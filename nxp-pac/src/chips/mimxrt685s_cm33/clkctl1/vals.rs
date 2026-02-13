#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp0fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x01,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x02,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Acmp0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Acmp0fclkselSel {
        Acmp0fclkselSel::from_bits(val)
    }
}
impl From<Acmp0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Acmp0fclkselSel) -> u8 {
        Acmp0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AudiomclkselSel {
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x0,
    #[doc = "AUDIO PLL Clock. (Shared Domain)"]
    AUDIO_PLL_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl AudiomclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AudiomclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AudiomclkselSel {
    #[inline(always)]
    fn from(val: u8) -> AudiomclkselSel {
        AudiomclkselSel::from_bits(val)
    }
}
impl From<AudiomclkselSel> for u8 {
    #[inline(always)]
    fn from(val: AudiomclkselSel) -> u8 {
        AudiomclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Audiopll0clkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "XTALIN Clock."]
    XTAL_CLK = 0x01,
    #[doc = "FFRO Clock Divided by 2."]
    FFRO_DIV_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Audiopll0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Audiopll0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Audiopll0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Audiopll0clkselSel {
        Audiopll0clkselSel::from_bits(val)
    }
}
impl From<Audiopll0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Audiopll0clkselSel) -> u8 {
        Audiopll0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bypass {
    #[doc = "PFD output is PFD programmed clock."]
    PROGRAMMED_CLK = 0x0,
    #[doc = "PFD output is AUDIOPLL0 reference input clock. (Bypass Mode)"]
    BYPASS = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkoutsel0Sel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "XTALIN Clock."]
    XTALIN_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "Main Clock."]
    MAIN_CLK = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "DSP Main Clock."]
    DSP_MAIN_CLK = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Clkoutsel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkoutsel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkoutsel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Clkoutsel0Sel {
        Clkoutsel0Sel::from_bits(val)
    }
}
impl From<Clkoutsel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Clkoutsel0Sel) -> u8 {
        Clkoutsel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkoutsel1Sel {
    #[doc = "CLKOUTSEL0 Multiplexed Output."]
    CLKOUTSEL0_OUTPUT = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLK = 0x02,
    #[doc = "DSP PLL clock."]
    DSP_PLL_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLK = 0x04,
    #[doc = "AUDIO PLL Clock."]
    AUDIO_PLL_CLK = 0x05,
    #[doc = "32KHz RTC Clock."]
    RTC_CLK_32KHZ = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Clkoutsel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkoutsel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkoutsel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Clkoutsel1Sel {
        Clkoutsel1Sel::from_bits(val)
    }
}
impl From<Clkoutsel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Clkoutsel1Sel) -> u8 {
        Clkoutsel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl CrcClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcClk {
    #[inline(always)]
    fn from(val: u8) -> CrcClk {
        CrcClk::from_bits(val)
    }
}
impl From<CrcClk> for u8 {
    #[inline(always)]
    fn from(val: CrcClk) -> u8 {
        CrcClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl CrcClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcClkClr {
    #[inline(always)]
    fn from(val: u8) -> CrcClkClr {
        CrcClkClr::from_bits(val)
    }
}
impl From<CrcClkClr> for u8 {
    #[inline(always)]
    fn from(val: CrcClkClr) -> u8 {
        CrcClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl CrcClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcClkSet {
    #[inline(always)]
    fn from(val: u8) -> CrcClkSet {
        CrcClkSet::from_bits(val)
    }
}
impl From<CrcClkSet> for u8 {
    #[inline(always)]
    fn from(val: CrcClkSet) -> u8 {
        CrcClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Ct32bit0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0Clk {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0Clk {
        Ct32bit0Clk::from_bits(val)
    }
}
impl From<Ct32bit0Clk> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0Clk) -> u8 {
        Ct32bit0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Ct32bit0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0ClkClr {
        Ct32bit0ClkClr::from_bits(val)
    }
}
impl From<Ct32bit0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0ClkClr) -> u8 {
        Ct32bit0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Ct32bit0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0ClkSet {
        Ct32bit0ClkSet::from_bits(val)
    }
}
impl From<Ct32bit0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0ClkSet) -> u8 {
        Ct32bit0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Ct32bit1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1Clk {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1Clk {
        Ct32bit1Clk::from_bits(val)
    }
}
impl From<Ct32bit1Clk> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1Clk) -> u8 {
        Ct32bit1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Ct32bit1ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1ClkClr {
        Ct32bit1ClkClr::from_bits(val)
    }
}
impl From<Ct32bit1ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1ClkClr) -> u8 {
        Ct32bit1ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Ct32bit1ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1ClkSet {
        Ct32bit1ClkSet::from_bits(val)
    }
}
impl From<Ct32bit1ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1ClkSet) -> u8 {
        Ct32bit1ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Ct32bit2Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2Clk {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2Clk {
        Ct32bit2Clk::from_bits(val)
    }
}
impl From<Ct32bit2Clk> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2Clk) -> u8 {
        Ct32bit2Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Ct32bit2ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2ClkClr {
        Ct32bit2ClkClr::from_bits(val)
    }
}
impl From<Ct32bit2ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2ClkClr) -> u8 {
        Ct32bit2ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Ct32bit2ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2ClkSet {
        Ct32bit2ClkSet::from_bits(val)
    }
}
impl From<Ct32bit2ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2ClkSet) -> u8 {
        Ct32bit2ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Ct32bit3Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3Clk {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3Clk {
        Ct32bit3Clk::from_bits(val)
    }
}
impl From<Ct32bit3Clk> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3Clk) -> u8 {
        Ct32bit3Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Ct32bit3ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3ClkClr {
        Ct32bit3ClkClr::from_bits(val)
    }
}
impl From<Ct32bit3ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3ClkClr) -> u8 {
        Ct32bit3ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Ct32bit3ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3ClkSet {
        Ct32bit3ClkSet::from_bits(val)
    }
}
impl From<Ct32bit3ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3ClkSet) -> u8 {
        Ct32bit3ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Ct32bit4Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4Clk {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4Clk {
        Ct32bit4Clk::from_bits(val)
    }
}
impl From<Ct32bit4Clk> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4Clk) -> u8 {
        Ct32bit4Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Ct32bit4ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4ClkClr {
        Ct32bit4ClkClr::from_bits(val)
    }
}
impl From<Ct32bit4ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4ClkClr) -> u8 {
        Ct32bit4ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Ct32bit4ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4ClkSet {
        Ct32bit4ClkSet::from_bits(val)
    }
}
impl From<Ct32bit4ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4ClkSet) -> u8 {
        Ct32bit4ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bitfclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x01,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x02,
    #[doc = "Audio PLL Clock."]
    AUDIO_PLL_CLK = 0x03,
    #[doc = "Master Clock In."]
    MASTER_CLK = 0x04,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Ct32bitfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bitfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bitfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Ct32bitfclkselSel {
        Ct32bitfclkselSel::from_bits(val)
    }
}
impl From<Ct32bitfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Ct32bitfclkselSel) -> u8 {
        Ct32bitfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Dmac0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0Clk {
    #[inline(always)]
    fn from(val: u8) -> Dmac0Clk {
        Dmac0Clk::from_bits(val)
    }
}
impl From<Dmac0Clk> for u8 {
    #[inline(always)]
    fn from(val: Dmac0Clk) -> u8 {
        Dmac0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Dmac0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ClkClr {
        Dmac0ClkClr::from_bits(val)
    }
}
impl From<Dmac0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ClkClr) -> u8 {
        Dmac0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Dmac0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ClkSet {
        Dmac0ClkSet::from_bits(val)
    }
}
impl From<Dmac0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ClkSet) -> u8 {
        Dmac0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Dmac1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1Clk {
    #[inline(always)]
    fn from(val: u8) -> Dmac1Clk {
        Dmac1Clk::from_bits(val)
    }
}
impl From<Dmac1Clk> for u8 {
    #[inline(always)]
    fn from(val: Dmac1Clk) -> u8 {
        Dmac1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Dmac1ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ClkClr {
        Dmac1ClkClr::from_bits(val)
    }
}
impl From<Dmac1ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ClkClr) -> u8 {
        Dmac1ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Dmac1ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ClkSet {
        Dmac1ClkSet::from_bits(val)
    }
}
impl From<Dmac1ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ClkSet) -> u8 {
        Dmac1ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Dmic0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0Clk {
    #[inline(always)]
    fn from(val: u8) -> Dmic0Clk {
        Dmic0Clk::from_bits(val)
    }
}
impl From<Dmic0Clk> for u8 {
    #[inline(always)]
    fn from(val: Dmic0Clk) -> u8 {
        Dmic0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Dmic0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Dmic0ClkClr {
        Dmic0ClkClr::from_bits(val)
    }
}
impl From<Dmic0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Dmic0ClkClr) -> u8 {
        Dmic0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Dmic0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Dmic0ClkSet {
        Dmic0ClkSet::from_bits(val)
    }
}
impl From<Dmic0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Dmic0ClkSet) -> u8 {
        Dmic0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0fclkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x01,
    #[doc = "Audio PLL Clock."]
    AUDIO_PLL_CLK = 0x02,
    #[doc = "Master Clock In."]
    MASTER_CLK = 0x03,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x04,
    #[doc = "32KHZ Wake Clk."]
    WAKE_CLK_32KHZ = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Dmic0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dmic0fclkselSel {
        Dmic0fclkselSel::from_bits(val)
    }
}
impl From<Dmic0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dmic0fclkselSel) -> u8 {
        Dmic0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspcpuclkselaSel {
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x0,
    #[doc = "XTALIN Clock."]
    XTAL_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x03,
}
impl DspcpuclkselaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspcpuclkselaSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspcpuclkselaSel {
    #[inline(always)]
    fn from(val: u8) -> DspcpuclkselaSel {
        DspcpuclkselaSel::from_bits(val)
    }
}
impl From<DspcpuclkselaSel> for u8 {
    #[inline(always)]
    fn from(val: DspcpuclkselaSel) -> u8 {
        DspcpuclkselaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspcpuclkselbSel {
    #[doc = "MAINCLKSELA 1st Stage Clock."]
    MAIN_1ST_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_PLL_CLK = 0x01,
    #[doc = "DSP System PLL Clock."]
    DSP_PLL_CLK = 0x02,
    #[doc = "RTC 32KHz Clock."]
    RTC_32K_CLK = 0x03,
}
impl DspcpuclkselbSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspcpuclkselbSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspcpuclkselbSel {
    #[inline(always)]
    fn from(val: u8) -> DspcpuclkselbSel {
        DspcpuclkselbSel::from_bits(val)
    }
}
impl From<DspcpuclkselbSel> for u8 {
    #[inline(always)]
    fn from(val: DspcpuclkselbSel) -> u8 {
        DspcpuclkselbSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dspmramclkdiv {
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 1."]
    DSP_CLK_DIV_BY_1 = 0x0,
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 2."]
    DSP_CLK_DIV_BY_2 = 0x01,
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 3."]
    DSP_CLK_DIV_BY_3 = 0x02,
    #[doc = "DSP MAINRAM Clk = DSP Core CLK / 4."]
    DSP_CLK_DIV_BY_4 = 0x03,
}
impl Dspmramclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dspmramclkdiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dspmramclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Dspmramclkdiv {
        Dspmramclkdiv::from_bits(val)
    }
}
impl From<Dspmramclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Dspmramclkdiv) -> u8 {
        Dspmramclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc0Clk {
        Fc0Clk::from_bits(val)
    }
}
impl From<Fc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc0Clk) -> u8 {
        Fc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc0ClkClr {
        Fc0ClkClr::from_bits(val)
    }
}
impl From<Fc0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc0ClkClr) -> u8 {
        Fc0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc0ClkSet {
        Fc0ClkSet::from_bits(val)
    }
}
impl From<Fc0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc0ClkSet) -> u8 {
        Fc0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc14SpiClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc14SpiClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc14SpiClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc14SpiClk {
    #[inline(always)]
    fn from(val: u8) -> Fc14SpiClk {
        Fc14SpiClk::from_bits(val)
    }
}
impl From<Fc14SpiClk> for u8 {
    #[inline(always)]
    fn from(val: Fc14SpiClk) -> u8 {
        Fc14SpiClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc14SpiClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc14SpiClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc14SpiClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc14SpiClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc14SpiClkClr {
        Fc14SpiClkClr::from_bits(val)
    }
}
impl From<Fc14SpiClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc14SpiClkClr) -> u8 {
        Fc14SpiClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc14SpiClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc14SpiClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc14SpiClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc14SpiClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc14SpiClkSet {
        Fc14SpiClkSet::from_bits(val)
    }
}
impl From<Fc14SpiClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc14SpiClkSet) -> u8 {
        Fc14SpiClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc14fclkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x01,
    #[doc = "Audio PLL Clock."]
    AUDIO_PLL_CLK = 0x02,
    #[doc = "Master Clock In."]
    MASTER_CLK = 0x03,
    #[doc = "FCn FRG Clock."]
    FCN_FRG_CLK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Fc14fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc14fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc14fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Fc14fclkselSel {
        Fc14fclkselSel::from_bits(val)
    }
}
impl From<Fc14fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Fc14fclkselSel) -> u8 {
        Fc14fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc15I2cClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cClk {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cClk {
        Fc15I2cClk::from_bits(val)
    }
}
impl From<Fc15I2cClk> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cClk) -> u8 {
        Fc15I2cClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc15I2cClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cClkClr {
        Fc15I2cClkClr::from_bits(val)
    }
}
impl From<Fc15I2cClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cClkClr) -> u8 {
        Fc15I2cClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15I2cClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc15I2cClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15I2cClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15I2cClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc15I2cClkSet {
        Fc15I2cClkSet::from_bits(val)
    }
}
impl From<Fc15I2cClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc15I2cClkSet) -> u8 {
        Fc15I2cClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc15fclkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x01,
    #[doc = "Audio PLL Clock."]
    AUDIO_PLL_CLK = 0x02,
    #[doc = "Master Clock In."]
    MASTER_CLK = 0x03,
    #[doc = "FCn FRG Clock."]
    FCN_FRG_CLK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Fc15fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc15fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc15fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Fc15fclkselSel {
        Fc15fclkselSel::from_bits(val)
    }
}
impl From<Fc15fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Fc15fclkselSel) -> u8 {
        Fc15fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc1Clk {
        Fc1Clk::from_bits(val)
    }
}
impl From<Fc1Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc1Clk) -> u8 {
        Fc1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc1ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc1ClkClr {
        Fc1ClkClr::from_bits(val)
    }
}
impl From<Fc1ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc1ClkClr) -> u8 {
        Fc1ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc1ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc1ClkSet {
        Fc1ClkSet::from_bits(val)
    }
}
impl From<Fc1ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc1ClkSet) -> u8 {
        Fc1ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc2Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc2Clk {
        Fc2Clk::from_bits(val)
    }
}
impl From<Fc2Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc2Clk) -> u8 {
        Fc2Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc2ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc2ClkClr {
        Fc2ClkClr::from_bits(val)
    }
}
impl From<Fc2ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc2ClkClr) -> u8 {
        Fc2ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc2ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc2ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc2ClkSet {
        Fc2ClkSet::from_bits(val)
    }
}
impl From<Fc2ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc2ClkSet) -> u8 {
        Fc2ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc3Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc3Clk {
        Fc3Clk::from_bits(val)
    }
}
impl From<Fc3Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc3Clk) -> u8 {
        Fc3Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc3ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc3ClkClr {
        Fc3ClkClr::from_bits(val)
    }
}
impl From<Fc3ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc3ClkClr) -> u8 {
        Fc3ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc3ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc3ClkSet {
        Fc3ClkSet::from_bits(val)
    }
}
impl From<Fc3ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc3ClkSet) -> u8 {
        Fc3ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc4Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc4Clk {
        Fc4Clk::from_bits(val)
    }
}
impl From<Fc4Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc4Clk) -> u8 {
        Fc4Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc4ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc4ClkClr {
        Fc4ClkClr::from_bits(val)
    }
}
impl From<Fc4ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc4ClkClr) -> u8 {
        Fc4ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc4ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc4ClkSet {
        Fc4ClkSet::from_bits(val)
    }
}
impl From<Fc4ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc4ClkSet) -> u8 {
        Fc4ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc5Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc5Clk {
        Fc5Clk::from_bits(val)
    }
}
impl From<Fc5Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc5Clk) -> u8 {
        Fc5Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc5ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc5ClkClr {
        Fc5ClkClr::from_bits(val)
    }
}
impl From<Fc5ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc5ClkClr) -> u8 {
        Fc5ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc5ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc5ClkSet {
        Fc5ClkSet::from_bits(val)
    }
}
impl From<Fc5ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc5ClkSet) -> u8 {
        Fc5ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc6Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc6Clk {
        Fc6Clk::from_bits(val)
    }
}
impl From<Fc6Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc6Clk) -> u8 {
        Fc6Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc6ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc6ClkClr {
        Fc6ClkClr::from_bits(val)
    }
}
impl From<Fc6ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc6ClkClr) -> u8 {
        Fc6ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc6ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc6ClkSet {
        Fc6ClkSet::from_bits(val)
    }
}
impl From<Fc6ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc6ClkSet) -> u8 {
        Fc6ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Fc7Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7Clk {
    #[inline(always)]
    fn from(val: u8) -> Fc7Clk {
        Fc7Clk::from_bits(val)
    }
}
impl From<Fc7Clk> for u8 {
    #[inline(always)]
    fn from(val: Fc7Clk) -> u8 {
        Fc7Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Fc7ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Fc7ClkClr {
        Fc7ClkClr::from_bits(val)
    }
}
impl From<Fc7ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Fc7ClkClr) -> u8 {
        Fc7ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Fc7ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Fc7ClkSet {
        Fc7ClkSet::from_bits(val)
    }
}
impl From<Fc7ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Fc7ClkSet) -> u8 {
        Fc7ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcfclkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x01,
    #[doc = "Audio PLL Clock."]
    AUDIO_PLL_CLK = 0x02,
    #[doc = "Master Clock In."]
    MASTER_CLK = 0x03,
    #[doc = "FCn FRG Clock."]
    FCN_FRG_CLK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl FcfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcfclkselSel {
        FcfclkselSel::from_bits(val)
    }
}
impl From<FcfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcfclkselSel) -> u8 {
        FcfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl FreqmeClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeClk {
    #[inline(always)]
    fn from(val: u8) -> FreqmeClk {
        FreqmeClk::from_bits(val)
    }
}
impl From<FreqmeClk> for u8 {
    #[inline(always)]
    fn from(val: FreqmeClk) -> u8 {
        FreqmeClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl FreqmeClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeClkClr {
    #[inline(always)]
    fn from(val: u8) -> FreqmeClkClr {
        FreqmeClkClr::from_bits(val)
    }
}
impl From<FreqmeClkClr> for u8 {
    #[inline(always)]
    fn from(val: FreqmeClkClr) -> u8 {
        FreqmeClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl FreqmeClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeClkSet {
    #[inline(always)]
    fn from(val: u8) -> FreqmeClkSet {
        FreqmeClkSet::from_bits(val)
    }
}
impl From<FreqmeClkSet> for u8 {
    #[inline(always)]
    fn from(val: FreqmeClkSet) -> u8 {
        FreqmeClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frg14clkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    FRG_PLL_CLK = 0x01,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Frg14clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frg14clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frg14clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Frg14clkselSel {
        Frg14clkselSel::from_bits(val)
    }
}
impl From<Frg14clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Frg14clkselSel) -> u8 {
        Frg14clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frg15clkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    FRG_PLL_CLK = 0x01,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Frg15clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frg15clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frg15clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Frg15clkselSel {
        Frg15clkselSel::from_bits(val)
    }
}
impl From<Frg15clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Frg15clkselSel) -> u8 {
        Frg15clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrgclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "FRG PLL Clock."]
    FRG_PLL_CLK = 0x01,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl FrgclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrgclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrgclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FrgclkselSel {
        FrgclkselSel::from_bits(val)
    }
}
impl From<FrgclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FrgclkselSel) -> u8 {
        FrgclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl GpiointctlClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlClk {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlClk {
        GpiointctlClk::from_bits(val)
    }
}
impl From<GpiointctlClk> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlClk) -> u8 {
        GpiointctlClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl GpiointctlClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlClkClr {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlClkClr {
        GpiointctlClkClr::from_bits(val)
    }
}
impl From<GpiointctlClkClr> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlClkClr) -> u8 {
        GpiointctlClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl GpiointctlClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlClkSet {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlClkSet {
        GpiointctlClkSet::from_bits(val)
    }
}
impl From<GpiointctlClkSet> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlClkSet) -> u8 {
        GpiointctlClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HoldringoffEna {
    #[doc = "disbale"]
    DSIABLE = 0x0,
    #[doc = "enable"]
    ENABLE = 0x01,
}
impl HoldringoffEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HoldringoffEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HoldringoffEna {
    #[inline(always)]
    fn from(val: u8) -> HoldringoffEna {
        HoldringoffEna::from_bits(val)
    }
}
impl From<HoldringoffEna> for u8 {
    #[inline(always)]
    fn from(val: HoldringoffEna) -> u8 {
        HoldringoffEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0Clk {
        Hsgpio0Clk::from_bits(val)
    }
}
impl From<Hsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0Clk) -> u8 {
        Hsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0ClkClr {
        Hsgpio0ClkClr::from_bits(val)
    }
}
impl From<Hsgpio0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0ClkClr) -> u8 {
        Hsgpio0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0ClkSet {
        Hsgpio0ClkSet::from_bits(val)
    }
}
impl From<Hsgpio0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0ClkSet) -> u8 {
        Hsgpio0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1Clk {
        Hsgpio1Clk::from_bits(val)
    }
}
impl From<Hsgpio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1Clk) -> u8 {
        Hsgpio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio1ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1ClkClr {
        Hsgpio1ClkClr::from_bits(val)
    }
}
impl From<Hsgpio1ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1ClkClr) -> u8 {
        Hsgpio1ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio1ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1ClkSet {
        Hsgpio1ClkSet::from_bits(val)
    }
}
impl From<Hsgpio1ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1ClkSet) -> u8 {
        Hsgpio1ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio2Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2Clk {
        Hsgpio2Clk::from_bits(val)
    }
}
impl From<Hsgpio2Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2Clk) -> u8 {
        Hsgpio2Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio2ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2ClkClr {
        Hsgpio2ClkClr::from_bits(val)
    }
}
impl From<Hsgpio2ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2ClkClr) -> u8 {
        Hsgpio2ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio2ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2ClkSet {
        Hsgpio2ClkSet::from_bits(val)
    }
}
impl From<Hsgpio2ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2ClkSet) -> u8 {
        Hsgpio2ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio3Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3Clk {
        Hsgpio3Clk::from_bits(val)
    }
}
impl From<Hsgpio3Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3Clk) -> u8 {
        Hsgpio3Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio3ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3ClkClr {
        Hsgpio3ClkClr::from_bits(val)
    }
}
impl From<Hsgpio3ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3ClkClr) -> u8 {
        Hsgpio3ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio3ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3ClkSet {
        Hsgpio3ClkSet::from_bits(val)
    }
}
impl From<Hsgpio3ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3ClkSet) -> u8 {
        Hsgpio3ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio4Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4Clk {
        Hsgpio4Clk::from_bits(val)
    }
}
impl From<Hsgpio4Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4Clk) -> u8 {
        Hsgpio4Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio4ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4ClkClr {
        Hsgpio4ClkClr::from_bits(val)
    }
}
impl From<Hsgpio4ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4ClkClr) -> u8 {
        Hsgpio4ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio4ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4ClkSet {
        Hsgpio4ClkSet::from_bits(val)
    }
}
impl From<Hsgpio4ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4ClkSet) -> u8 {
        Hsgpio4ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio5Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5Clk {
        Hsgpio5Clk::from_bits(val)
    }
}
impl From<Hsgpio5Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5Clk) -> u8 {
        Hsgpio5Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio5ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5ClkClr {
        Hsgpio5ClkClr::from_bits(val)
    }
}
impl From<Hsgpio5ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5ClkClr) -> u8 {
        Hsgpio5ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio5ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5ClkSet {
        Hsgpio5ClkSet::from_bits(val)
    }
}
impl From<Hsgpio5ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5ClkSet) -> u8 {
        Hsgpio5ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio6Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6Clk {
        Hsgpio6Clk::from_bits(val)
    }
}
impl From<Hsgpio6Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6Clk) -> u8 {
        Hsgpio6Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio6ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6ClkClr {
        Hsgpio6ClkClr::from_bits(val)
    }
}
impl From<Hsgpio6ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6ClkClr) -> u8 {
        Hsgpio6ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio6ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6ClkSet {
        Hsgpio6ClkSet::from_bits(val)
    }
}
impl From<Hsgpio6ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6ClkSet) -> u8 {
        Hsgpio6ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Hsgpio7Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7Clk {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7Clk {
        Hsgpio7Clk::from_bits(val)
    }
}
impl From<Hsgpio7Clk> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7Clk) -> u8 {
        Hsgpio7Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Hsgpio7ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7ClkClr {
        Hsgpio7ClkClr::from_bits(val)
    }
}
impl From<Hsgpio7ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7ClkClr) -> u8 {
        Hsgpio7ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Hsgpio7ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7ClkSet {
        Hsgpio7ClkSet::from_bits(val)
    }
}
impl From<Hsgpio7ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7ClkSet) -> u8 {
        Hsgpio7ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl I3c0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0Clk {
    #[inline(always)]
    fn from(val: u8) -> I3c0Clk {
        I3c0Clk::from_bits(val)
    }
}
impl From<I3c0Clk> for u8 {
    #[inline(always)]
    fn from(val: I3c0Clk) -> u8 {
        I3c0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl I3c0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> I3c0ClkClr {
        I3c0ClkClr::from_bits(val)
    }
}
impl From<I3c0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: I3c0ClkClr) -> u8 {
        I3c0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl I3c0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> I3c0ClkSet {
        I3c0ClkSet::from_bits(val)
    }
}
impl From<I3c0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: I3c0ClkSet) -> u8 {
        I3c0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl I3c0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkselSel {
        I3c0fclkselSel::from_bits(val)
    }
}
impl From<I3c0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkselSel) -> u8 {
        I3c0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcselSel {
    #[doc = "I3C0 FCLK Selection."]
    I3C0_FCLK_SELECTION = 0x0,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl I3c0fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcselSel {
        I3c0fclkstcselSel::from_bits(val)
    }
}
impl From<I3c0fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcselSel) -> u8 {
        I3c0fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Mrt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Mrt0Clk {
        Mrt0Clk::from_bits(val)
    }
}
impl From<Mrt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Mrt0Clk) -> u8 {
        Mrt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Mrt0ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Mrt0ClkClr {
        Mrt0ClkClr::from_bits(val)
    }
}
impl From<Mrt0ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Mrt0ClkClr) -> u8 {
        Mrt0ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Mrt0ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Mrt0ClkSet {
        Mrt0ClkSet::from_bits(val)
    }
}
impl From<Mrt0ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Mrt0ClkSet) -> u8 {
        Mrt0ClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl MuClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuClk {
    #[inline(always)]
    fn from(val: u8) -> MuClk {
        MuClk::from_bits(val)
    }
}
impl From<MuClk> for u8 {
    #[inline(always)]
    fn from(val: MuClk) -> u8 {
        MuClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl MuClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuClkClr {
    #[inline(always)]
    fn from(val: u8) -> MuClkClr {
        MuClkClr::from_bits(val)
    }
}
impl From<MuClkClr> for u8 {
    #[inline(always)]
    fn from(val: MuClkClr) -> u8 {
        MuClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl MuClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuClkSet {
    #[inline(always)]
    fn from(val: u8) -> MuClkSet {
        MuClkSet::from_bits(val)
    }
}
impl From<MuClkSet> for u8 {
    #[inline(always)]
    fn from(val: MuClkSet) -> u8 {
        MuClkSet::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mult(u8);
impl Mult {
    #[doc = "Div 16"]
    pub const DIV_16: Self = Self(0x10);
    #[doc = "Div 17"]
    pub const DIV_17: Self = Self(0x11);
    #[doc = "Div 20"]
    pub const DIV_20: Self = Self(0x14);
    #[doc = "Div 22"]
    pub const DIV_22: Self = Self(0x16);
    #[doc = "Div 27"]
    pub const DIV_27: Self = Self(0x1b);
    #[doc = "Div 33"]
    pub const DIV_33: Self = Self(0x21);
}
impl Mult {
    pub const fn from_bits(val: u8) -> Mult {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Mult {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x10 => f.write_str("DIV_16"),
            0x11 => f.write_str("DIV_17"),
            0x14 => f.write_str("DIV_20"),
            0x16 => f.write_str("DIV_22"),
            0x1b => f.write_str("DIV_27"),
            0x21 => f.write_str("DIV_33"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mult {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x10 => defmt::write!(f, "DIV_16"),
            0x11 => defmt::write!(f, "DIV_17"),
            0x14 => defmt::write!(f, "DIV_20"),
            0x16 => defmt::write!(f, "DIV_22"),
            0x1b => defmt::write!(f, "DIV_27"),
            0x21 => defmt::write!(f, "DIV_33"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Mult {
    #[inline(always)]
    fn from(val: u8) -> Mult {
        Mult::from_bits(val)
    }
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(val: Mult) -> u8 {
        Mult::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OseventTimerClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl OseventTimerClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OseventTimerClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OseventTimerClk {
    #[inline(always)]
    fn from(val: u8) -> OseventTimerClk {
        OseventTimerClk::from_bits(val)
    }
}
impl From<OseventTimerClk> for u8 {
    #[inline(always)]
    fn from(val: OseventTimerClk) -> u8 {
        OseventTimerClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OseventTimerClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl OseventTimerClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OseventTimerClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OseventTimerClkClr {
    #[inline(always)]
    fn from(val: u8) -> OseventTimerClkClr {
        OseventTimerClkClr::from_bits(val)
    }
}
impl From<OseventTimerClkClr> for u8 {
    #[inline(always)]
    fn from(val: OseventTimerClkClr) -> u8 {
        OseventTimerClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OseventTimerClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl OseventTimerClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OseventTimerClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OseventTimerClkSet {
    #[inline(always)]
    fn from(val: u8) -> OseventTimerClkSet {
        OseventTimerClkSet::from_bits(val)
    }
}
impl From<OseventTimerClkSet> for u8 {
    #[inline(always)]
    fn from(val: OseventTimerClkSet) -> u8 {
        OseventTimerClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OseventfclkselSel {
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x0,
    #[doc = "RTC 32KHz Clock."]
    RTC_32KHZ_CLK = 0x01,
    #[doc = "Teal Free Running Clock (Global Time Stamping)"]
    TEAL_FREE_RUNNING_CLK = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl OseventfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OseventfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OseventfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> OseventfclkselSel {
        OseventfclkselSel::from_bits(val)
    }
}
impl From<OseventfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: OseventfclkselSel) -> u8 {
        OseventfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PimctlClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl PimctlClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlClk {
    #[inline(always)]
    fn from(val: u8) -> PimctlClk {
        PimctlClk::from_bits(val)
    }
}
impl From<PimctlClk> for u8 {
    #[inline(always)]
    fn from(val: PimctlClk) -> u8 {
        PimctlClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PimctlClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl PimctlClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlClkClr {
    #[inline(always)]
    fn from(val: u8) -> PimctlClkClr {
        PimctlClkClr::from_bits(val)
    }
}
impl From<PimctlClkClr> for u8 {
    #[inline(always)]
    fn from(val: PimctlClkClr) -> u8 {
        PimctlClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PimctlClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl PimctlClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlClkSet {
    #[inline(always)]
    fn from(val: u8) -> PimctlClkSet {
        PimctlClkSet::from_bits(val)
    }
}
impl From<PimctlClkSet> for u8 {
    #[inline(always)]
    fn from(val: PimctlClkSet) -> u8 {
        PimctlClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reset {
    #[doc = "AUDIOPLL0 reset is removed."]
    NORMAL = 0x0,
    #[doc = "AUDIOPLL0 is placed into reset."]
    RESET = 0x01,
}
impl Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reset {
    #[inline(always)]
    fn from(val: u8) -> Reset {
        Reset::from_bits(val)
    }
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(val: Reset) -> u8 {
        Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcLiteClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl RtcLiteClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcLiteClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcLiteClk {
    #[inline(always)]
    fn from(val: u8) -> RtcLiteClk {
        RtcLiteClk::from_bits(val)
    }
}
impl From<RtcLiteClk> for u8 {
    #[inline(always)]
    fn from(val: RtcLiteClk) -> u8 {
        RtcLiteClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcLiteClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl RtcLiteClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcLiteClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcLiteClkClr {
    #[inline(always)]
    fn from(val: u8) -> RtcLiteClkClr {
        RtcLiteClkClr::from_bits(val)
    }
}
impl From<RtcLiteClkClr> for u8 {
    #[inline(always)]
    fn from(val: RtcLiteClkClr) -> u8 {
        RtcLiteClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcLiteClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl RtcLiteClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcLiteClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcLiteClkSet {
    #[inline(always)]
    fn from(val: u8) -> RtcLiteClkSet {
        RtcLiteClkSet::from_bits(val)
    }
}
impl From<RtcLiteClkSet> for u8 {
    #[inline(always)]
    fn from(val: RtcLiteClkSet) -> u8 {
        RtcLiteClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl SemaClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaClk {
    #[inline(always)]
    fn from(val: u8) -> SemaClk {
        SemaClk::from_bits(val)
    }
}
impl From<SemaClk> for u8 {
    #[inline(always)]
    fn from(val: SemaClk) -> u8 {
        SemaClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl SemaClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaClkClr {
    #[inline(always)]
    fn from(val: u8) -> SemaClkClr {
        SemaClkClr::from_bits(val)
    }
}
impl From<SemaClkClr> for u8 {
    #[inline(always)]
    fn from(val: SemaClkClr) -> u8 {
        SemaClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl SemaClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaClkSet {
    #[inline(always)]
    fn from(val: u8) -> SemaClkSet {
        SemaClkSet::from_bits(val)
    }
}
impl From<SemaClkSet> for u8 {
    #[inline(always)]
    fn from(val: SemaClkSet) -> u8 {
        SemaClkSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1fclkselSel {
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Wdt1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt1fclkselSel {
        Wdt1fclkselSel::from_bits(val)
    }
}
impl From<Wdt1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt1fclkselSel) -> u8 {
        Wdt1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Wwdt1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1Clk {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1Clk {
        Wwdt1Clk::from_bits(val)
    }
}
impl From<Wwdt1Clk> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1Clk) -> u8 {
        Wwdt1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1ClkClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Wwdt1ClkClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1ClkClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1ClkClr {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1ClkClr {
        Wwdt1ClkClr::from_bits(val)
    }
}
impl From<Wwdt1ClkClr> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1ClkClr) -> u8 {
        Wwdt1ClkClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1ClkSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Wwdt1ClkSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1ClkSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1ClkSet {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1ClkSet {
        Wwdt1ClkSet::from_bits(val)
    }
}
impl From<Wwdt1ClkSet> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1ClkSet) -> u8 {
        Wwdt1ClkSet::to_bits(val)
    }
}
