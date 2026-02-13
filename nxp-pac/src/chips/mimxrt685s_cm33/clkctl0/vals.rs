#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0fclksel0Sel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "XTALIN Clock."]
    XTALIN_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Adc0fclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0fclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0fclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Adc0fclksel0Sel {
        Adc0fclksel0Sel::from_bits(val)
    }
}
impl From<Adc0fclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Adc0fclksel0Sel) -> u8 {
        Adc0fclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0fclksel1Sel {
    #[doc = "ADC0FCLKSEL0 Multiplexed Output."]
    ADC0FCLKSEL0_MUX_OUT = 0x0,
    #[doc = "SYSPLL0 MAIN_CLK (PFD0 Output)"]
    SYSPLL0_MAIN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Adc0fclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0fclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0fclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Adc0fclksel1Sel {
        Adc0fclksel1Sel::from_bits(val)
    }
}
impl From<Adc0fclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Adc0fclksel1Sel) -> u8 {
        Adc0fclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bypass {
    #[doc = "PFD output is PFD programmed clock."]
    PROGRAMMED_CLK = 0x0,
    #[doc = "PFD output is PLL Input clock. (Bypass)"]
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
pub enum BypassEnable {
    #[doc = "Normal Mode."]
    NORMAL_MODE = 0x0,
    #[doc = "Bypass Mode."]
    BYPASS_MODE = 0x01,
}
impl BypassEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BypassEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BypassEnable {
    #[inline(always)]
    fn from(val: u8) -> BypassEnable {
        BypassEnable::from_bits(val)
    }
}
impl From<BypassEnable> for u8 {
    #[inline(always)]
    fn from(val: BypassEnable) -> u8 {
        BypassEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Espifclksel0Sel {
    #[doc = "FFRO Clock (48/60m_irc)."]
    FFRO_CLK = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Espifclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Espifclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Espifclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Espifclksel0Sel {
        Espifclksel0Sel::from_bits(val)
    }
}
impl From<Espifclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Espifclksel0Sel) -> u8 {
        Espifclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspifclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl FlexspifclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspifclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspifclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexspifclkselSel {
        FlexspifclkselSel::from_bits(val)
    }
}
impl From<FlexspifclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexspifclkselSel) -> u8 {
        FlexspifclkselSel::to_bits(val)
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
pub enum LpEnable {
    #[doc = "High Gain Mode(HP)."]
    HP = 0x0,
    #[doc = "Low Power mode (LP)."]
    LP = 0x01,
}
impl LpEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpEnable {
    #[inline(always)]
    fn from(val: u8) -> LpEnable {
        LpEnable::from_bits(val)
    }
}
impl From<LpEnable> for u8 {
    #[inline(always)]
    fn from(val: LpEnable) -> u8 {
        LpEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselaSel {
    #[doc = "FFRO Clock Divided by 4."]
    FFRO_DIV_4 = 0x0,
    #[doc = "SYSXTALIN Clock."]
    SYSXTAL_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
}
impl MainclkselaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselaSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselaSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselaSel {
        MainclkselaSel::from_bits(val)
    }
}
impl From<MainclkselaSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselaSel) -> u8 {
        MainclkselaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkselbSel {
    #[doc = "MAINCLKSELA 1st Stage Clock."]
    MAIN_1ST_CLK = 0x0,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x01,
    #[doc = "Main System PLL Clock."]
    MAIN_PLL_CLK = 0x02,
    #[doc = "RTC 32KHz Clock."]
    RTC_32K_CLK = 0x03,
}
impl MainclkselbSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselbSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselbSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselbSel {
        MainclkselbSel::from_bits(val)
    }
}
impl From<MainclkselbSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselbSel) -> u8 {
        MainclkselbSel::to_bits(val)
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
pub enum Pscctl0CasperClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0CasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0CasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0CasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0CasperClk {
        Pscctl0CasperClk::from_bits(val)
    }
}
impl From<Pscctl0CasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0CasperClk) -> u8 {
        Pscctl0CasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrCasperClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrCasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrCasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrCasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrCasperClk {
        Pscctl0ClrCasperClk::from_bits(val)
    }
}
impl From<Pscctl0ClrCasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrCasperClk) -> u8 {
        Pscctl0ClrCasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrFlexspiOtfadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrFlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrFlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrFlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrFlexspiOtfadClk {
        Pscctl0ClrFlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0ClrFlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrFlexspiOtfadClk) -> u8 {
        Pscctl0ClrFlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrHashcryptClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrHashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrHashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrHashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrHashcryptClk {
        Pscctl0ClrHashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0ClrHashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrHashcryptClk) -> u8 {
        Pscctl0ClrHashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrOtpClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrOtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrOtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrOtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrOtpClk {
        Pscctl0ClrOtpClk::from_bits(val)
    }
}
impl From<Pscctl0ClrOtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrOtpClk) -> u8 {
        Pscctl0ClrOtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrPowerquadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrPowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrPowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrPowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrPowerquadClk {
        Pscctl0ClrPowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0ClrPowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrPowerquadClk) -> u8 {
        Pscctl0ClrPowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrPufClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrPufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrPufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrPufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrPufClk {
        Pscctl0ClrPufClk::from_bits(val)
    }
}
impl From<Pscctl0ClrPufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrPufClk) -> u8 {
        Pscctl0ClrPufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrRngClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrRngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrRngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrRngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrRngClk {
        Pscctl0ClrRngClk::from_bits(val)
    }
}
impl From<Pscctl0ClrRngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrRngClk) -> u8 {
        Pscctl0ClrRngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrRomCtl128kbClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrRomCtl128kbClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrRomCtl128kbClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrRomCtl128kbClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrRomCtl128kbClk {
        Pscctl0ClrRomCtl128kbClk::from_bits(val)
    }
}
impl From<Pscctl0ClrRomCtl128kbClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrRomCtl128kbClk) -> u8 {
        Pscctl0ClrRomCtl128kbClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrSctClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrSctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrSctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrSctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrSctClk {
        Pscctl0ClrSctClk::from_bits(val)
    }
}
impl From<Pscctl0ClrSctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrSctClk) -> u8 {
        Pscctl0ClrSctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrUsbhsDeviceClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsDeviceClk {
        Pscctl0ClrUsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsDeviceClk) -> u8 {
        Pscctl0ClrUsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrUsbhsHostClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsHostClk {
        Pscctl0ClrUsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsHostClk) -> u8 {
        Pscctl0ClrUsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrUsbhsPhyClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsPhyClk {
        Pscctl0ClrUsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsPhyClk) -> u8 {
        Pscctl0ClrUsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0ClrUsbhsSramClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsSramClk {
        Pscctl0ClrUsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsSramClk) -> u8 {
        Pscctl0ClrUsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0FlexspiOtfadClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0FlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0FlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0FlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0FlexspiOtfadClk {
        Pscctl0FlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0FlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0FlexspiOtfadClk) -> u8 {
        Pscctl0FlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0HashcryptClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0HashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0HashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0HashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0HashcryptClk {
        Pscctl0HashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0HashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0HashcryptClk) -> u8 {
        Pscctl0HashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0OtpClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0OtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0OtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0OtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0OtpClk {
        Pscctl0OtpClk::from_bits(val)
    }
}
impl From<Pscctl0OtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0OtpClk) -> u8 {
        Pscctl0OtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0PowerquadClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0PowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0PowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0PowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0PowerquadClk {
        Pscctl0PowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0PowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0PowerquadClk) -> u8 {
        Pscctl0PowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0PufClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0PufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0PufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0PufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0PufClk {
        Pscctl0PufClk::from_bits(val)
    }
}
impl From<Pscctl0PufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0PufClk) -> u8 {
        Pscctl0PufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0RngClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0RngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0RngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0RngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0RngClk {
        Pscctl0RngClk::from_bits(val)
    }
}
impl From<Pscctl0RngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0RngClk) -> u8 {
        Pscctl0RngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SctClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0SctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SctClk {
        Pscctl0SctClk::from_bits(val)
    }
}
impl From<Pscctl0SctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SctClk) -> u8 {
        Pscctl0SctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetCasperClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetCasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetCasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetCasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetCasperClk {
        Pscctl0SetCasperClk::from_bits(val)
    }
}
impl From<Pscctl0SetCasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetCasperClk) -> u8 {
        Pscctl0SetCasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetFlexspiOtfadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetFlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetFlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetFlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetFlexspiOtfadClk {
        Pscctl0SetFlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0SetFlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetFlexspiOtfadClk) -> u8 {
        Pscctl0SetFlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetHashcryptClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetHashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetHashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetHashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetHashcryptClk {
        Pscctl0SetHashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0SetHashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetHashcryptClk) -> u8 {
        Pscctl0SetHashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetOtpClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetOtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetOtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetOtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetOtpClk {
        Pscctl0SetOtpClk::from_bits(val)
    }
}
impl From<Pscctl0SetOtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetOtpClk) -> u8 {
        Pscctl0SetOtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetPowerquadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetPowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetPowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetPowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetPowerquadClk {
        Pscctl0SetPowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0SetPowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetPowerquadClk) -> u8 {
        Pscctl0SetPowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetPufClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetPufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetPufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetPufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetPufClk {
        Pscctl0SetPufClk::from_bits(val)
    }
}
impl From<Pscctl0SetPufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetPufClk) -> u8 {
        Pscctl0SetPufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetRngClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetRngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetRngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetRngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetRngClk {
        Pscctl0SetRngClk::from_bits(val)
    }
}
impl From<Pscctl0SetRngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetRngClk) -> u8 {
        Pscctl0SetRngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetRomCtl128kbClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetRomCtl128kbClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetRomCtl128kbClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetRomCtl128kbClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetRomCtl128kbClk {
        Pscctl0SetRomCtl128kbClk::from_bits(val)
    }
}
impl From<Pscctl0SetRomCtl128kbClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetRomCtl128kbClk) -> u8 {
        Pscctl0SetRomCtl128kbClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetSctClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetSctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetSctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetSctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetSctClk {
        Pscctl0SetSctClk::from_bits(val)
    }
}
impl From<Pscctl0SetSctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetSctClk) -> u8 {
        Pscctl0SetSctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetUsbhsDeviceClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsDeviceClk {
        Pscctl0SetUsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsDeviceClk) -> u8 {
        Pscctl0SetUsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetUsbhsHostClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsHostClk {
        Pscctl0SetUsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsHostClk) -> u8 {
        Pscctl0SetUsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetUsbhsPhyClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsPhyClk {
        Pscctl0SetUsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsPhyClk) -> u8 {
        Pscctl0SetUsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0SetUsbhsSramClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsSramClk {
        Pscctl0SetUsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsSramClk) -> u8 {
        Pscctl0SetUsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0UsbhsDeviceClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsDeviceClk {
        Pscctl0UsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsDeviceClk) -> u8 {
        Pscctl0UsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0UsbhsHostClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsHostClk {
        Pscctl0UsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsHostClk) -> u8 {
        Pscctl0UsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0UsbhsPhyClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsPhyClk {
        Pscctl0UsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsPhyClk) -> u8 {
        Pscctl0UsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl0UsbhsSramClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsSramClk {
        Pscctl0UsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsSramClk) -> u8 {
        Pscctl0UsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1Acmp0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Acmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Acmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Acmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Acmp0Clk {
        Pscctl1Acmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1Acmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Acmp0Clk) -> u8 {
        Pscctl1Acmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1Adc0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Adc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Adc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Adc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Adc0Clk {
        Pscctl1Adc0Clk::from_bits(val)
    }
}
impl From<Pscctl1Adc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Adc0Clk) -> u8 {
        Pscctl1Adc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1ClrAcmp0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrAcmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrAcmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrAcmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrAcmp0Clk {
        Pscctl1ClrAcmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrAcmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrAcmp0Clk) -> u8 {
        Pscctl1ClrAcmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1ClrAdc0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrAdc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrAdc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrAdc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrAdc0Clk {
        Pscctl1ClrAdc0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrAdc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrAdc0Clk) -> u8 {
        Pscctl1ClrAdc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1ClrSdio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrSdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrSdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrSdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrSdio0Clk {
        Pscctl1ClrSdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrSdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrSdio0Clk) -> u8 {
        Pscctl1ClrSdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1ClrSdio1Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrSdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrSdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrSdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrSdio1Clk {
        Pscctl1ClrSdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrSdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrSdio1Clk) -> u8 {
        Pscctl1ClrSdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1ClrShsgpio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrShsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrShsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrShsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrShsgpio0Clk {
        Pscctl1ClrShsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrShsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrShsgpio0Clk) -> u8 {
        Pscctl1ClrShsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1Sdio0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Sdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Sdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Sdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Sdio0Clk {
        Pscctl1Sdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1Sdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Sdio0Clk) -> u8 {
        Pscctl1Sdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1Sdio1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Sdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Sdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Sdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Sdio1Clk {
        Pscctl1Sdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1Sdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Sdio1Clk) -> u8 {
        Pscctl1Sdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1SetAcmp0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetAcmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetAcmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetAcmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetAcmp0Clk {
        Pscctl1SetAcmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetAcmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetAcmp0Clk) -> u8 {
        Pscctl1SetAcmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1SetAdc0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetAdc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetAdc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetAdc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetAdc0Clk {
        Pscctl1SetAdc0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetAdc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetAdc0Clk) -> u8 {
        Pscctl1SetAdc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1SetSdio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetSdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetSdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetSdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetSdio0Clk {
        Pscctl1SetSdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetSdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetSdio0Clk) -> u8 {
        Pscctl1SetSdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1SetSdio1Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetSdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetSdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetSdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetSdio1Clk {
        Pscctl1SetSdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1SetSdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetSdio1Clk) -> u8 {
        Pscctl1SetSdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1SetShsgpio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetShsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetShsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetShsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetShsgpio0Clk {
        Pscctl1SetShsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetShsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetShsgpio0Clk) -> u8 {
        Pscctl1SetShsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl1Shsgpio0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Shsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Shsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Shsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Shsgpio0Clk {
        Pscctl1Shsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1Shsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Shsgpio0Clk) -> u8 {
        Pscctl1Shsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2ClrUtick0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl2ClrUtick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2ClrUtick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2ClrUtick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2ClrUtick0Clk {
        Pscctl2ClrUtick0Clk::from_bits(val)
    }
}
impl From<Pscctl2ClrUtick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2ClrUtick0Clk) -> u8 {
        Pscctl2ClrUtick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2ClrWwdt0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl2ClrWwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2ClrWwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2ClrWwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2ClrWwdt0Clk {
        Pscctl2ClrWwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2ClrWwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2ClrWwdt0Clk) -> u8 {
        Pscctl2ClrWwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2SetUtick0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl2SetUtick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2SetUtick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2SetUtick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2SetUtick0Clk {
        Pscctl2SetUtick0Clk::from_bits(val)
    }
}
impl From<Pscctl2SetUtick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2SetUtick0Clk) -> u8 {
        Pscctl2SetUtick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2SetWwdt0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl2SetWwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2SetWwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2SetWwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2SetWwdt0Clk {
        Pscctl2SetWwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2SetWwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2SetWwdt0Clk) -> u8 {
        Pscctl2SetWwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2Utick0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl2Utick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2Utick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2Utick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2Utick0Clk {
        Pscctl2Utick0Clk::from_bits(val)
    }
}
impl From<Pscctl2Utick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2Utick0Clk) -> u8 {
        Pscctl2Utick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pscctl2Wwdt0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl2Wwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2Wwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2Wwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2Wwdt0Clk {
        Pscctl2Wwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2Wwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2Wwdt0Clk) -> u8 {
        Pscctl2Wwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reset {
    #[doc = "SYSPLL0 reset is removed."]
    NORMAL = 0x0,
    #[doc = "SYSPLL0 is placed into reset."]
    FORCED_INTO_RESET = 0x01,
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
pub enum RomCtl128kb {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl RomCtl128kb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomCtl128kb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomCtl128kb {
    #[inline(always)]
    fn from(val: u8) -> RomCtl128kb {
        RomCtl128kb::from_bits(val)
    }
}
impl From<RomCtl128kb> for u8 {
    #[inline(always)]
    fn from(val: RomCtl128kb) -> u8 {
        RomCtl128kb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctfclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    #[doc = "AUDIO PLL Clock"]
    AUDIO_PLL_CLK = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SctfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctfclkselSel {
        SctfclkselSel::from_bits(val)
    }
}
impl From<SctfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctfclkselSel) -> u8 {
        SctfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdio0fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Sdio0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sdio0fclkselSel {
        Sdio0fclkselSel::from_bits(val)
    }
}
impl From<Sdio0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sdio0fclkselSel) -> u8 {
        Sdio0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdio1fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Sdio1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sdio1fclkselSel {
        Sdio1fclkselSel::from_bits(val)
    }
}
impl From<Sdio1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sdio1fclkselSel) -> u8 {
        Sdio1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysoscbypassSel {
    #[doc = "External XTAL Clock."]
    EXT_XTAL_CLK = 0x0,
    #[doc = "Clock IN Clock."]
    CLOCK_IN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "NONE.this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SysoscbypassSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysoscbypassSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysoscbypassSel {
    #[inline(always)]
    fn from(val: u8) -> SysoscbypassSel {
        SysoscbypassSel::from_bits(val)
    }
}
impl From<SysoscbypassSel> for u8 {
    #[inline(always)]
    fn from(val: SysoscbypassSel) -> u8 {
        SysoscbypassSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syspll0clkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "SYSXTALIN Clock."]
    SYSXTAL_CLK = 0x01,
    #[doc = "FFRO Clock Divided by 2."]
    FFRO_DIV_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Syspll0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syspll0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syspll0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Syspll0clkselSel {
        Syspll0clkselSel::from_bits(val)
    }
}
impl From<Syspll0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Syspll0clkselSel) -> u8 {
        Syspll0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystickfclkselSel {
    #[doc = "Systick Divider Output Clock."]
    SYSTICK_DIV_CLK = 0x0,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x01,
    #[doc = "32KHz RTC Clock."]
    RTC_32KHZ = 0x02,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SystickfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystickfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystickfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SystickfclkselSel {
        SystickfclkselSel::from_bits(val)
    }
}
impl From<SystickfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SystickfclkselSel) -> u8 {
        SystickfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimRange {
    #[doc = "48MHz."]
    FFRO_48MHZ = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "60MHz."]
    FFRO_60MHZ = 0x03,
}
impl TrimRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimRange {
    #[inline(always)]
    fn from(val: u8) -> TrimRange {
        TrimRange::from_bits(val)
    }
}
impl From<TrimRange> for u8 {
    #[inline(always)]
    fn from(val: TrimRange) -> u8 {
        TrimRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Update {
    #[doc = "Normal Mode."]
    NORMAL_MODE = 0x0,
    #[doc = "Update Safe Mode."]
    UPDATE_SAFE_MODE = 0x01,
}
impl Update {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Update {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Update {
    #[inline(always)]
    fn from(val: u8) -> Update {
        Update::from_bits(val)
    }
}
impl From<Update> for u8 {
    #[inline(always)]
    fn from(val: Update) -> u8 {
        Update::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbhsfclkselSel {
    #[doc = "XTALIN Clock."]
    XTALIN_CLK = 0x0,
    #[doc = "Main Clock."]
    MAIN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl UsbhsfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbhsfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbhsfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UsbhsfclkselSel {
        UsbhsfclkselSel::from_bits(val)
    }
}
impl From<UsbhsfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UsbhsfclkselSel) -> u8 {
        UsbhsfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickfclkselSel {
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
impl UtickfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickfclkselSel {
        UtickfclkselSel::from_bits(val)
    }
}
impl From<UtickfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickfclkselSel) -> u8 {
        UtickfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakeclk32khzselSel {
    #[doc = "32KHz"]
    FREQ_32KHZ = 0x0,
    #[doc = "LPOSC (Divided by 32 by default)."]
    LPOSC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Wakeclk32khzselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeclk32khzselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeclk32khzselSel {
    #[inline(always)]
    fn from(val: u8) -> Wakeclk32khzselSel {
        Wakeclk32khzselSel::from_bits(val)
    }
}
impl From<Wakeclk32khzselSel> for u8 {
    #[inline(always)]
    fn from(val: Wakeclk32khzselSel) -> u8 {
        Wakeclk32khzselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0fclkselSel {
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
impl Wdt0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt0fclkselSel {
        Wdt0fclkselSel::from_bits(val)
    }
}
impl From<Wdt0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt0fclkselSel) -> u8 {
        Wdt0fclkselSel::to_bits(val)
    }
}
