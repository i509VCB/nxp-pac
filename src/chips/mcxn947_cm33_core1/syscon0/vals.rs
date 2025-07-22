#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Adc0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkselSel {
        Adc0clkselSel::from_bits(val)
    }
}
impl From<Adc0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkselSel) -> u8 {
        Adc0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in clock"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Adc1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkselSel {
        Adc1clkselSel::from_bits(val)
    }
}
impl From<Adc1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkselSel) -> u8 {
        Adc1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AssetProtection {
    #[doc = "ELS asset is protected"]
    VALUE0 = 0x0,
    #[doc = "ELS asset is not protected"]
    VALUE1 = 0x01,
    #[doc = "ELS asset is protected"]
    VALUE2 = 0x02,
    #[doc = "ELS asset is protected"]
    VALUE3 = 0x03,
}
impl AssetProtection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AssetProtection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AssetProtection {
    #[inline(always)]
    fn from(val: u8) -> AssetProtection {
        AssetProtection::from_bits(val)
    }
}
impl From<AssetProtection> for u8 {
    #[inline(always)]
    fn from(val: AssetProtection) -> u8 {
        AssetProtection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootImage {
    #[doc = "Internal flash image 0"]
    ENUM0 = 0x0,
    #[doc = "Internal flash image 1"]
    ENUM1 = 0x01,
    #[doc = "FlexSPI flash image 0"]
    ENUM2 = 0x02,
    #[doc = "FlexSPI flash image 1"]
    ENUM3 = 0x03,
    #[doc = "Recovery SPI flash image"]
    ENUM4 = 0x04,
    #[doc = "Serial boot image (write-memory and execute ISP command used)"]
    ENUM5 = 0x05,
    #[doc = "Receive SB3 containing SB_JUMP command is used."]
    ENUM6 = 0x06,
    #[doc = "Customer SBL/recovery image (Bank1 IFR0)."]
    ENUM7 = 0x07,
    #[doc = "NXP MAD recovery image (Bank1 IFR0)."]
    ENUM8 = 0x08,
    #[doc = "NXP ROM extension (NMPA - Bank0 IFR0)."]
    ENUM9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl BootImage {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootImage {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootImage {
    #[inline(always)]
    fn from(val: u8) -> BootImage {
        BootImage::from_bits(val)
    }
}
impl From<BootImage> for u8 {
    #[inline(always)]
    fn from(val: BootImage) -> u8 {
        BootImage::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock (main_clk)"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock (pll0_clk)"]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock (clk_in)"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock (fro_hf)"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 12 MHz clock (fro_12m)"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock (pll1_clk)"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock (lp_osc)"]
    ENUM_0X6 = 0x06,
    #[doc = "USB PLL clock (usb_pll_clk)"]
    ENUM_0X7 = 0x07,
    #[doc = "No clock"]
    ENUM_0X8 = 0x08,
    #[doc = "No clock"]
    ENUM_0X9 = 0x09,
    #[doc = "No clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "No clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "No clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrFlashCache {
    #[doc = "No clear flash cache"]
    ENABLE = 0x0,
    #[doc = "Clears flash cache"]
    DISABLE = 0x01,
}
impl ClrFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrFlashCache {
    #[inline(always)]
    fn from(val: u8) -> ClrFlashCache {
        ClrFlashCache::from_bits(val)
    }
}
impl From<ClrFlashCache> for u8 {
    #[inline(always)]
    fn from(val: ClrFlashCache) -> u8 {
        ClrFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache"]
    ENABLE = 0x0,
    #[doc = "Clears the cache"]
    DISABLE = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0fclkselSel {
        Cmp0fclkselSel::from_bits(val)
    }
}
impl From<Cmp0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0fclkselSel) -> u8 {
        Cmp0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp0rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkselSel {
        Cmp0rrclkselSel::from_bits(val)
    }
}
impl From<Cmp0rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkselSel) -> u8 {
        Cmp0rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1fclkselSel {
        Cmp1fclkselSel::from_bits(val)
    }
}
impl From<Cmp1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1fclkselSel) -> u8 {
        Cmp1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp1rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkselSel {
        Cmp1rrclkselSel::from_bits(val)
    }
}
impl From<Cmp1rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkselSel) -> u8 {
        Cmp1rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp2fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp2fclkselSel {
        Cmp2fclkselSel::from_bits(val)
    }
}
impl From<Cmp2fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp2fclkselSel) -> u8 {
        Cmp2fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "FRO_HF clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_12M clock"]
    ENUM3 = 0x03,
    #[doc = "CLKIN clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Cmp2rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkselSel {
        Cmp2rrclkselSel::from_bits(val)
    }
}
impl From<Cmp2rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkselSel) -> u8 {
        Cmp2rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup"]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup"]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF = 0x01,
}
impl Cpu0nstckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalNoref {
        Cpu0nstckcalNoref::from_bits(val)
    }
}
impl From<Cpu0nstckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalNoref) -> u8 {
        Cpu0nstckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT = 0x01,
}
impl Cpu0nstckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalSkew {
        Cpu0nstckcalSkew::from_bits(val)
    }
}
impl From<Cpu0nstckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalSkew) -> u8 {
        Cpu0nstckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping"]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping"]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF = 0x01,
}
impl Cpu0stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalNoref {
        Cpu0stckcalNoref::from_bits(val)
    }
}
impl From<Cpu0stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalNoref) -> u8 {
        Cpu0stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT = 0x01,
}
impl Cpu0stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalSkew {
        Cpu0stckcalSkew::from_bits(val)
    }
}
impl From<Cpu0stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalSkew) -> u8 {
        Cpu0stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1lockup {
    #[doc = "CPU is not in lockup"]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup"]
    SLEEPING = 0x01,
}
impl Cpu1lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu1lockup {
        Cpu1lockup::from_bits(val)
    }
}
impl From<Cpu1lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu1lockup) -> u8 {
        Cpu1lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1rsten {
    #[doc = "The CPU1 is not reset."]
    RELEASED = 0x0,
    #[doc = "The CPU1 is reset."]
    ASSERTED = 0x01,
}
impl Cpu1rsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1rsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1rsten {
    #[inline(always)]
    fn from(val: u8) -> Cpu1rsten {
        Cpu1rsten::from_bits(val)
    }
}
impl From<Cpu1rsten> for u8 {
    #[inline(always)]
    fn from(val: Cpu1rsten) -> u8 {
        Cpu1rsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1sleeping {
    #[doc = "CPU is not sleeping"]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping"]
    SLEEPING = 0x01,
}
impl Cpu1sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu1sleeping {
        Cpu1sleeping::from_bits(val)
    }
}
impl From<Cpu1sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu1sleeping) -> u8 {
        Cpu1sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalNoref {
    #[doc = "Reference clock is provided"]
    YES_REF_1 = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF_1 = 0x01,
}
impl Cpu1stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalNoref {
        Cpu1stckcalNoref::from_bits(val)
    }
}
impl From<Cpu1stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalNoref) -> u8 {
        Cpu1stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalSkew {
    #[doc = "TENMS value is exact"]
    EXACT_1 = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT_1 = 0x01,
}
impl Cpu1stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalSkew {
        Cpu1stckcalSkew::from_bits(val)
    }
}
impl From<Cpu1stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalSkew) -> u8 {
        Cpu1stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkselSel {
    #[doc = "FRO_1M clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO 12MHz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "SAI0 MCLK IN clock"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
    #[doc = "SAI1 MCLK IN clock"]
    ENUM_0X8 = 0x08,
    #[doc = "SAI0 TX_BCLK clock"]
    ENUM_0X9 = 0x09,
    #[doc = "SAI0 RX_BCLK clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "SAI1 TX_BCLK clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "SAI1 RX_BCLK clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl CtimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkselSel {
        CtimerclkselSel::from_bits(val)
    }
}
impl From<CtimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkselSel) -> u8 {
        CtimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac0clkselSel {
        Dac0clkselSel::from_bits(val)
    }
}
impl From<Dac0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac0clkselSel) -> u8 {
        Dac0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac1clkselSel {
        Dac1clkselSel::from_bits(val)
    }
}
impl From<Dac1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac1clkselSel) -> u8 {
        Dac1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "Clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12M"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Dac2clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Dac2clkselSel {
        Dac2clkselSel::from_bits(val)
    }
}
impl From<Dac2clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Dac2clkselSel) -> u8 {
        Dac2clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Dbgen {
        DebugFeaturesCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Dbgen) -> u8 {
        DebugFeaturesCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Niden {
        DebugFeaturesCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Niden) -> u8 {
        DebugFeaturesCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        DebugFeaturesDpCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Dbgen) -> u8 {
        DebugFeaturesDpCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Niden {
        DebugFeaturesDpCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Niden) -> u8 {
        DebugFeaturesDpCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation"]
    ENABLE = 0x0,
    #[doc = "Disables data speculation"]
    DISABLE = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashCache {
    #[doc = "Enables flash cache"]
    ENABLE = 0x0,
    #[doc = "Disables flash cache"]
    DISABLE = 0x01,
}
impl DisFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashCache {
    #[inline(always)]
    fn from(val: u8) -> DisFlashCache {
        DisFlashCache::from_bits(val)
    }
}
impl From<DisFlashCache> for u8 {
    #[inline(always)]
    fn from(val: DisFlashCache) -> u8 {
        DisFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashData {
    #[doc = "Enables flash data cache when DIS_FLASH_CACHE=0"]
    ENABLE = 0x0,
    #[doc = "Disables flash data cache"]
    DISABLE = 0x01,
}
impl DisFlashData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashData {
    #[inline(always)]
    fn from(val: u8) -> DisFlashData {
        DisFlashData::from_bits(val)
    }
}
impl From<DisFlashData> for u8 {
    #[inline(always)]
    fn from(val: DisFlashData) -> u8 {
        DisFlashData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashInst {
    #[doc = "Enables flash instruction cache when DIS_FLASH_CACHE=0"]
    ENABLE = 0x0,
    #[doc = "Disables flash instruction cache"]
    DISABLE = 0x01,
}
impl DisFlashInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashInst {
    #[inline(always)]
    fn from(val: u8) -> DisFlashInst {
        DisFlashInst::from_bits(val)
    }
}
impl From<DisFlashInst> for u8 {
    #[inline(always)]
    fn from(val: DisFlashInst) -> u8 {
        DisFlashInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation"]
    ENABLE = 0x0,
    #[doc = "Disables flash speculation"]
    DISABLE = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data"]
    DISABLE = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction"]
    DISABLE = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgden {
    #[inline(always)]
    fn from(val: u8) -> DspDbgden {
        DspDbgden::from_bits(val)
    }
}
impl From<DspDbgden> for u8 {
    #[inline(always)]
    fn from(val: DspDbgden) -> u8 {
        DspDbgden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug"]
    DISABLE = 0x01,
    #[doc = "Enables debug"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgen {
    #[inline(always)]
    fn from(val: u8) -> DspDbgen {
        DspDbgen::from_bits(val)
    }
}
impl From<DspDbgen> for u8 {
    #[inline(always)]
    fn from(val: DspDbgen) -> u8 {
        DspDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Emvsim0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkselSel {
        Emvsim0clkselSel::from_bits(val)
    }
}
impl From<Emvsim0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkselSel) -> u8 {
        Emvsim0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock0"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Emvsim1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkselSel {
        Emvsim1clkselSel::from_bits(val)
    }
}
impl From<Emvsim1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkselSel) -> u8 {
        Emvsim1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "enet0_tx_clk clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clk1 clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl EnetptprefclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkselSel {
        EnetptprefclkselSel::from_bits(val)
    }
}
impl From<EnetptprefclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkselSel) -> u8 {
        EnetptprefclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl EnetrmiiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkselSel {
        EnetrmiiclkselSel::from_bits(val)
    }
}
impl From<EnetrmiiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkselSel) -> u8 {
        EnetrmiiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0clkselSel {
    #[doc = "clk_16k\\[2\\]"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
}
impl Ewm0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0clkselSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Ewm0clkselSel {
        Ewm0clkselSel::from_bits(val)
    }
}
impl From<Ewm0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Ewm0clkselSel) -> u8 {
        Ewm0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL divided clock"]
    ENUM_0X1 = 0x01,
    #[doc = "FRO 12 MHz clock"]
    ENUM_0X2 = 0x02,
    #[doc = "fro_hf_div clock"]
    ENUM_0X3 = 0x03,
    #[doc = "clk_1m clock"]
    ENUM_0X4 = 0x04,
    #[doc = "USB PLL clock"]
    ENUM_0X5 = 0x05,
    #[doc = "LP Oscillator clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
    #[doc = "No clock"]
    ENUM8 = 0x08,
    #[doc = "No clock"]
    ENUM9 = 0x09,
    #[doc = "No clock"]
    ENUM10 = 0x0a,
    #[doc = "No clock"]
    ENUM11 = 0x0b,
    #[doc = "No clock"]
    ENUM12 = 0x0c,
    #[doc = "No clock"]
    ENUM13 = 0x0d,
    #[doc = "No clock"]
    ENUM14 = 0x0e,
    #[doc = "No clock"]
    ENUM15 = 0x0f,
}
impl FlexSpiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkselSel {
        FlexSpiclkselSel::from_bits(val)
    }
}
impl From<FlexSpiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkselSel) -> u8 {
        FlexSpiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Flexcan0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkselSel {
        Flexcan0clkselSel::from_bits(val)
    }
}
impl From<Flexcan0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkselSel) -> u8 {
        Flexcan0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Flexcan1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkselSel {
        Flexcan1clkselSel::from_bits(val)
    }
}
impl From<Flexcan1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkselSel) -> u8 {
        Flexcan1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl FlexioclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkselSel {
        FlexioclkselSel::from_bits(val)
    }
}
impl From<FlexioclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkselSel) -> u8 {
        FlexioclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation"]
    ENABLE = 0x0,
    #[doc = "Forces no allocation"]
    DISABLE = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetIsoSw {
    #[doc = "Isolation is disabled"]
    DISABLE0 = 0x0,
    #[doc = "Isolation is disabled"]
    DISABLE1 = 0x01,
    #[doc = "Isolation is enabled. When both GDET0_CTRL/GDET1_CTRL GDET_ISO_SW are \"10\", isolation_on is asserted."]
    ENABLE = 0x02,
    #[doc = "Isolation is disabled"]
    DISABLE3 = 0x03,
}
impl GdetIsoSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetIsoSw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetIsoSw {
    #[inline(always)]
    fn from(val: u8) -> GdetIsoSw {
        GdetIsoSw::from_bits(val)
    }
}
impl From<GdetIsoSw> for u8 {
    #[inline(always)]
    fn from(val: GdetIsoSw) -> u8 {
        GdetIsoSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
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
pub enum I3c0fclksselSel {
    #[doc = "FRO_1M clock"]
    ENUM0 = 0x0,
    #[doc = "No clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c0fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksselSel {
        I3c0fclksselSel::from_bits(val)
    }
}
impl From<I3c0fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksselSel) -> u8 {
        I3c0fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcselSel {
    #[doc = "I3C0 functional clock I3C0FCLK"]
    ENUM0 = 0x0,
    #[doc = "FRO_1M clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
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
pub enum I3c1fclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkselSel {
        I3c1fclkselSel::from_bits(val)
    }
}
impl From<I3c1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkselSel) -> u8 {
        I3c1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksselSel {
    #[doc = "FRO_1M clock"]
    ENUM0 = 0x0,
    #[doc = "No clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksselSel {
        I3c1fclksselSel::from_bits(val)
    }
}
impl From<I3c1fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksselSel) -> u8 {
        I3c1fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcselSel {
    #[doc = "I3C1 functional clock I3C1FCLK"]
    ENUM0 = 0x0,
    #[doc = "FRO_1M clock"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl I3c1fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcselSel {
        I3c1fclkstcselSel::from_bits(val)
    }
}
impl From<I3c1fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcselSel) -> u8 {
        I3c1fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is interleaved. This setting is need for PKC L0 memory access."]
    INTERLEAVE = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers"]
    ENABLE = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkselSel {
    #[doc = "FRO_12M clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_clk0 clock"]
    ENUM4 = 0x04,
    #[doc = "SAI0_MCLK clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
    #[doc = "SAI1_MCLK clock"]
    ENUM8 = 0x08,
    #[doc = "No clock"]
    ENUM9 = 0x09,
    #[doc = "No clock"]
    ENUM10 = 0x0a,
    #[doc = "No clock"]
    ENUM11 = 0x0b,
    #[doc = "No clock"]
    ENUM12 = 0x0c,
    #[doc = "No clock"]
    ENUM13 = 0x0d,
    #[doc = "No clock"]
    ENUM14 = 0x0e,
    #[doc = "No clock"]
    ENUM15 = 0x0f,
}
impl MicfilfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkselSel {
        MicfilfclkselSel::from_bits(val)
    }
}
impl From<MicfilfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkselSel) -> u8 {
        MicfilfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerclkselSel {
    #[doc = "clk_16k\\[2\\]"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
}
impl OstimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> OstimerclkselSel {
        OstimerclkselSel::from_bits(val)
    }
}
impl From<OstimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: OstimerclkselSel) -> u8 {
        OstimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhySel {
    #[doc = "Selects MII PHY Interface"]
    MII = 0x0,
    #[doc = "Selects RMII PHY Interface"]
    RMII = 0x01,
}
impl PhySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhySel {
    #[inline(always)]
    fn from(val: u8) -> PhySel {
        PhySel::from_bits(val)
    }
}
impl From<PhySel> for u8 {
    #[inline(always)]
    fn from(val: PhySel) -> u8 {
        PhySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivselSel {
    #[doc = "PLL0 clock"]
    ENUM0 = 0x0,
    #[doc = "pll1_clk0"]
    ENUM1 = 0x01,
    #[doc = "No clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
    #[doc = "No clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "No clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl PllclkdivselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivselSel {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivselSel {
        PllclkdivselSel::from_bits(val)
    }
}
impl From<PllclkdivselSel> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivselSel) -> u8 {
        PllclkdivselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxI {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxI {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxI {
        PriCoolfluxI::from_bits(val)
    }
}
impl From<PriCoolfluxI> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxI) -> u8 {
        PriCoolfluxI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxX {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxX {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxX {
        PriCoolfluxX::from_bits(val)
    }
}
impl From<PriCoolfluxX> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxX) -> u8 {
        PriCoolfluxX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxYEspi {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxYEspi {
        PriCoolfluxYEspi::from_bits(val)
    }
}
impl From<PriCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxYEspi) -> u8 {
        PriCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Cbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Cbus {
        PriCpu0Cbus::from_bits(val)
    }
}
impl From<PriCpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Cbus) -> u8 {
        PriCpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Sbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Sbus {
        PriCpu0Sbus::from_bits(val)
    }
}
impl From<PriCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Sbus) -> u8 {
        PriCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1CbusSmartDmaI {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1CbusSmartDmaI {
        PriCpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<PriCpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1CbusSmartDmaI) -> u8 {
        PriCpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1SbusSmartDmaD {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriCpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1SbusSmartDmaD {
        PriCpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<PriCpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1SbusSmartDmaD) -> u8 {
        PriCpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuD {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriNpuD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuD {
    #[inline(always)]
    fn from(val: u8) -> PriNpuD {
        PriNpuD::from_bits(val)
    }
}
impl From<PriNpuD> for u8 {
    #[inline(always)]
    fn from(val: PriNpuD) -> u8 {
        PriNpuD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuPq {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriNpuPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuPq {
    #[inline(always)]
    fn from(val: u8) -> PriNpuPq {
        PriNpuPq::from_bits(val)
    }
}
impl From<PriNpuPq> for u8 {
    #[inline(always)]
    fn from(val: PriNpuPq) -> u8 {
        PriNpuPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriPkcEls {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriPkcEls {
    #[inline(always)]
    fn from(val: u8) -> PriPkcEls {
        PriPkcEls::from_bits(val)
    }
}
impl From<PriPkcEls> for u8 {
    #[inline(always)]
    fn from(val: PriPkcEls) -> u8 {
        PriPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbFsEnet {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> PriUsbFsEnet {
        PriUsbFsEnet::from_bits(val)
    }
}
impl From<PriUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: PriUsbFsEnet) -> u8 {
        PriUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbHs {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbHs {
    #[inline(always)]
    fn from(val: u8) -> PriUsbHs {
        PriUsbHs::from_bits(val)
    }
}
impl From<PriUsbHs> for u8 {
    #[inline(always)]
    fn from(val: PriUsbHs) -> u8 {
        PriUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsdhc {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl PriUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsdhc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsdhc {
    #[inline(always)]
    fn from(val: u8) -> PriUsdhc {
        PriUsdhc::from_bits(val)
    }
}
impl From<PriUsdhc> for u8 {
    #[inline(always)]
    fn from(val: PriUsdhc) -> u8 {
        PriUsdhc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prot(u16);
impl Prot {
    #[doc = "For write operation to have an effect."]
    pub const PROT: Self = Self(0xc0c4);
}
impl Prot {
    pub const fn from_bits(val: u16) -> Prot {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Prot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xc0c4 => f.write_str("PROT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prot {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xc0c4 => defmt::write!(f, "PROT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Prot {
    #[inline(always)]
    fn from(val: u16) -> Prot {
        Prot::from_bits(val)
    }
}
impl From<Prot> for u16 {
    #[inline(always)]
    fn from(val: Prot) -> u16 {
        Prot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rspt {
    #[doc = "No response when the ETB count expires"]
    NO_RESP = 0x0,
    #[doc = "Generates a normal interrupt when the ETB count expires"]
    INTERRUPT = 0x01,
    #[doc = "Generates an NMI interrupt when the ETB count expires"]
    NMI = 0x02,
    #[doc = "Generates a debug halt when the ETB count expires via CPU0 CTICHIN\\[2\\]"]
    DEBUG_HALT = 0x03,
}
impl Rspt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rspt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rspt {
    #[inline(always)]
    fn from(val: u8) -> Rspt {
        Rspt::from_bits(val)
    }
}
impl From<Rspt> for u8 {
    #[inline(always)]
    fn from(val: Rspt) -> u8 {
        Rspt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_CLK0 clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Sai0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkselSel {
        Sai0clkselSel::from_bits(val)
    }
}
impl From<Sai0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkselSel) -> u8 {
        Sai0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "PLL1_CLK0 clock"]
    ENUM4 = 0x04,
    #[doc = "No clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl Sai1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkselSel {
        Sai1clkselSel::from_bits(val)
    }
}
impl From<Sai1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkselSel) -> u8 {
        Sai1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sb3 {
    #[doc = "customer fw load/update file."]
    CUSTOMER = 0x0,
    #[doc = "NXP Provisioning FW."]
    NXP = 0x01,
    #[doc = "ELS signed OEM Provisioning FW."]
    OEM = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sb3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sb3 {
    #[inline(always)]
    fn from(val: u8) -> Sb3 {
        Sb3::from_bits(val)
    }
}
impl From<Sb3> for u8 {
    #[inline(always)]
    fn from(val: Sb3) -> u8 {
        Sb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X4 = 0x04,
    #[doc = "SAI0 MCLK_IN clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
    #[doc = "SAI1 MCLK_IN clock"]
    ENUM_0X8 = 0x08,
    #[doc = "No clock"]
    ENUM_0X9 = 0x09,
    #[doc = "No clock"]
    ENUM_0X_A = 0x0a,
    #[doc = "No clock"]
    ENUM_0X_B = 0x0b,
    #[doc = "No clock"]
    ENUM_0X_C = 0x0c,
    #[doc = "No clock"]
    ENUM_0X_D = 0x0d,
    #[doc = "No clock"]
    ENUM_0X_E = 0x0e,
    #[doc = "No clock"]
    ENUM_0X_F = 0x0f,
}
impl SctclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctclkselSel {
        SctclkselSel::from_bits(val)
    }
}
impl From<SctclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctclkselSel) -> u8 {
        SctclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincfiltclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12Mhz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl SincfiltclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincfiltclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincfiltclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SincfiltclkselSel {
        SincfiltclkselSel::from_bits(val)
    }
}
impl From<SincfiltclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SincfiltclkselSel) -> u8 {
        SincfiltclkselSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu0SecCode(u32);
impl SwdAccessCpu0SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessCpu0SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu0SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu0SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu0SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu0SecCode {
        SwdAccessCpu0SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu0SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu0SecCode) -> u32 {
        SwdAccessCpu0SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu1SecCode(u32);
impl SwdAccessCpu1SecCode {
    #[doc = "CPU1 DAP is not allowed"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Security code to allow CPU1 DAP"]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessCpu1SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu1SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu1SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu1SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu1SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu1SecCode {
        SwdAccessCpu1SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu1SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu1SecCode) -> u32 {
        SwdAccessCpu1SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessDspSecCode(u32);
impl SwdAccessDspSecCode {
    #[doc = "DSP DAP is not allowed. Reading back register is read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable DSP SWD access. Reading back register is read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SwdAccessDspSecCode {
    pub const fn from_bits(val: u32) -> SwdAccessDspSecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessDspSecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessDspSecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessDspSecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessDspSecCode {
        SwdAccessDspSecCode::from_bits(val)
    }
}
impl From<SwdAccessDspSecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessDspSecCode) -> u32 {
        SwdAccessDspSecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "SYSTICKCLKDIV0 output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel1Sel {
    #[doc = "SYSTICKCLKDIV1 output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Systickclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel1Sel {
        Systickclksel1Sel::from_bits(val)
    }
}
impl From<Systickclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel1Sel) -> u8 {
        Systickclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "TRACECLKDIV output"]
    ENUM_0X0 = 0x0,
    #[doc = "Clk 1 MHz clock"]
    ENUM_0X1 = 0x01,
    #[doc = "LP Oscillator clock"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "No clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "No clock"]
    ENUM_0X1 = 0x01,
    #[doc = "clk_in"]
    ENUM_0X2 = 0x02,
    #[doc = "No clock"]
    ENUM_0X3 = 0x03,
    #[doc = "FRO_12Mhz clock"]
    ENUM_0X4 = 0x04,
    #[doc = "No clock"]
    ENUM_0X5 = 0x05,
    #[doc = "No clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl TsiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TsiclkselSel {
        TsiclkselSel::from_bits(val)
    }
}
impl From<TsiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TsiclkselSel) -> u8 {
        TsiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkselSel {
    #[doc = "No clock"]
    ENUM0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM1 = 0x01,
    #[doc = "CLKIN clock"]
    ENUM2 = 0x02,
    #[doc = "FRO_HF clock"]
    ENUM3 = 0x03,
    #[doc = "FRO_12M clock"]
    ENUM4 = 0x04,
    #[doc = "pll1_clk1 clock"]
    ENUM5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM6 = 0x06,
    #[doc = "No clock"]
    ENUM7 = 0x07,
}
impl USdhcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkselSel {
        USdhcclkselSel::from_bits(val)
    }
}
impl From<USdhcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkselSel) -> u8 {
        USdhcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers"]
    ENABLE = 0x0,
    #[doc = "Freezes all clock configuration registers update"]
    FREEZE = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkselSel {
    #[doc = "No clock"]
    ENUM_0X0 = 0x0,
    #[doc = "PLL0 clock"]
    ENUM_0X1 = 0x01,
    #[doc = "No clock"]
    ENUM_0X2 = 0x02,
    #[doc = "Clk 48 MHz clock"]
    ENUM_0X3 = 0x03,
    #[doc = "Clk_in"]
    ENUM_0X4 = 0x04,
    #[doc = "PLL1_clk0 clock"]
    ENUM_0X5 = 0x05,
    #[doc = "USB PLL clock"]
    ENUM_0X6 = 0x06,
    #[doc = "No clock"]
    ENUM_0X7 = 0x07,
}
impl Usb0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkselSel {
        Usb0clkselSel::from_bits(val)
    }
}
impl From<Usb0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkselSel) -> u8 {
        Usb0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkselSel {
    #[doc = "clk_in"]
    ENUM0 = 0x0,
    #[doc = "xtal32k\\[2\\]"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "No clock"]
    ENUM3 = 0x03,
}
impl UtickclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickclkselSel {
        UtickclkselSel::from_bits(val)
    }
}
impl From<UtickclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickclkselSel) -> u8 {
        UtickclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkselSel {
    #[doc = "FRO16K clock 2"]
    ENUM0 = 0x0,
    #[doc = "fro_hf_div clock"]
    ENUM1 = 0x01,
    #[doc = "clk_1m clock"]
    ENUM2 = 0x02,
    #[doc = "clk_1m clock"]
    ENUM3 = 0x03,
}
impl Wdt1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkselSel {
        Wdt1clkselSel::from_bits(val)
    }
}
impl From<Wdt1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkselSel) -> u8 {
        Wdt1clkselSel::to_bits(val)
    }
}
