#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V, 240 Ohm for DDR)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V__240_OHM_FOR_DDR_ = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
}
impl Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dse {
    #[inline(always)]
    fn from(val: u8) -> Dse {
        Dse::from_bits(val)
    }
}
impl From<Dse> for u8 {
    #[inline(always)]
    fn from(val: Dse) -> u8 {
        Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmaSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_SD_02 for Mode: ALT2"]
    GPIO_SD_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_02 for Mode: ALT2"]
    GPIO_02_ALT2 = 0x01,
}
impl Flexpwm1PwmaSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmaSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmaSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmaSelectInput0Daisy {
        Flexpwm1PwmaSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmaSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmaSelectInput0Daisy) -> u8 {
        Flexpwm1PwmaSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmaSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_SD_04 for Mode: ALT2"]
    GPIO_SD_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_04 for Mode: ALT2"]
    GPIO_04_ALT2 = 0x01,
}
impl Flexpwm1PwmaSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmaSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmaSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmaSelectInput1Daisy {
        Flexpwm1PwmaSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmaSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmaSelectInput1Daisy) -> u8 {
        Flexpwm1PwmaSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmaSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT2"]
    GPIO_AD_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_06 for Mode: ALT2"]
    GPIO_06_ALT2 = 0x01,
}
impl Flexpwm1PwmaSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmaSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmaSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmaSelectInput2Daisy {
        Flexpwm1PwmaSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmaSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmaSelectInput2Daisy) -> u8 {
        Flexpwm1PwmaSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmaSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT2"]
    GPIO_AD_06_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_08 for Mode: ALT2"]
    GPIO_08_ALT2 = 0x01,
}
impl Flexpwm1PwmaSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmaSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmaSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmaSelectInput3Daisy {
        Flexpwm1PwmaSelectInput3Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmaSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmaSelectInput3Daisy) -> u8 {
        Flexpwm1PwmaSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmbSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_SD_01 for Mode: ALT2"]
    GPIO_SD_01_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_01 for Mode: ALT2"]
    GPIO_01_ALT2 = 0x01,
}
impl Flexpwm1PwmbSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmbSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmbSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmbSelectInput0Daisy {
        Flexpwm1PwmbSelectInput0Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmbSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmbSelectInput0Daisy) -> u8 {
        Flexpwm1PwmbSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmbSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_SD_03 for Mode: ALT2"]
    GPIO_SD_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_03 for Mode: ALT2"]
    GPIO_03_ALT2 = 0x01,
}
impl Flexpwm1PwmbSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmbSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmbSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmbSelectInput1Daisy {
        Flexpwm1PwmbSelectInput1Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmbSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmbSelectInput1Daisy) -> u8 {
        Flexpwm1PwmbSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmbSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT2"]
    GPIO_AD_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_05 for Mode: ALT2"]
    GPIO_05_ALT2 = 0x01,
}
impl Flexpwm1PwmbSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmbSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmbSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmbSelectInput2Daisy {
        Flexpwm1PwmbSelectInput2Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmbSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmbSelectInput2Daisy) -> u8 {
        Flexpwm1PwmbSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1PwmbSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT2"]
    GPIO_AD_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_07 for Mode: ALT2"]
    GPIO_07_ALT2 = 0x01,
}
impl Flexpwm1PwmbSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1PwmbSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1PwmbSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1PwmbSelectInput3Daisy {
        Flexpwm1PwmbSelectInput3Daisy::from_bits(val)
    }
}
impl From<Flexpwm1PwmbSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1PwmbSelectInput3Daisy) -> u8 {
        Flexpwm1PwmbSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiDqsFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_14 for Mode: ALT0"]
    GPIO_SD_14_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT0"]
    GPIO_SD_12_ALT0 = 0x01,
}
impl FlexspiDqsFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiDqsFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiDqsFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiDqsFaSelectInputDaisy {
        FlexspiDqsFaSelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiDqsFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiDqsFaSelectInputDaisy) -> u8 {
        FlexspiDqsFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiDqsFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_14 for Mode: ALT1"]
    GPIO_SD_14_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_00 for Mode: ALT0"]
    GPIO_00_ALT0 = 0x01,
}
impl FlexspiDqsFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiDqsFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiDqsFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiDqsFbSelectInputDaisy {
        FlexspiDqsFbSelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiDqsFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiDqsFbSelectInputDaisy) -> u8 {
        FlexspiDqsFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppColSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT2"]
    GPIO_AD_14_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT2"]
    GPIO_12_ALT2 = 0x01,
}
impl KppColSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppColSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppColSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppColSelectInput0Daisy {
        KppColSelectInput0Daisy::from_bits(val)
    }
}
impl From<KppColSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppColSelectInput0Daisy) -> u8 {
        KppColSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppColSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT2"]
    GPIO_AD_12_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT3"]
    GPIO_AD_06_ALT3 = 0x01,
}
impl KppColSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppColSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppColSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppColSelectInput1Daisy {
        KppColSelectInput1Daisy::from_bits(val)
    }
}
impl From<KppColSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppColSelectInput1Daisy) -> u8 {
        KppColSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppColSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT2"]
    GPIO_AD_10_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT3"]
    GPIO_AD_04_ALT3 = 0x01,
}
impl KppColSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppColSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppColSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppColSelectInput2Daisy {
        KppColSelectInput2Daisy::from_bits(val)
    }
}
impl From<KppColSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppColSelectInput2Daisy) -> u8 {
        KppColSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppColSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT2"]
    GPIO_AD_00_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_02 for Mode: ALT4"]
    GPIO_02_ALT4 = 0x01,
}
impl KppColSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppColSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppColSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppColSelectInput3Daisy {
        KppColSelectInput3Daisy::from_bits(val)
    }
}
impl From<KppColSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppColSelectInput3Daisy) -> u8 {
        KppColSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppRowSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT2"]
    GPIO_AD_13_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_11 for Mode: ALT2"]
    GPIO_11_ALT2 = 0x01,
}
impl KppRowSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppRowSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppRowSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppRowSelectInput0Daisy {
        KppRowSelectInput0Daisy::from_bits(val)
    }
}
impl From<KppRowSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppRowSelectInput0Daisy) -> u8 {
        KppRowSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppRowSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_AD_11 for Mode: ALT2"]
    GPIO_AD_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT3"]
    GPIO_AD_05_ALT3 = 0x01,
}
impl KppRowSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppRowSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppRowSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppRowSelectInput1Daisy {
        KppRowSelectInput1Daisy::from_bits(val)
    }
}
impl From<KppRowSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppRowSelectInput1Daisy) -> u8 {
        KppRowSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppRowSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT2"]
    GPIO_AD_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT3"]
    GPIO_AD_03_ALT3 = 0x01,
}
impl KppRowSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppRowSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppRowSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppRowSelectInput2Daisy {
        KppRowSelectInput2Daisy::from_bits(val)
    }
}
impl From<KppRowSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppRowSelectInput2Daisy) -> u8 {
        KppRowSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KppRowSelectInput3Daisy {
    #[doc = "Selecting Pad: GPIO_13 for Mode: ALT2"]
    GPIO_13_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_01 for Mode: ALT4"]
    GPIO_01_ALT4 = 0x01,
}
impl KppRowSelectInput3Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KppRowSelectInput3Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KppRowSelectInput3Daisy {
    #[inline(always)]
    fn from(val: u8) -> KppRowSelectInput3Daisy {
        KppRowSelectInput3Daisy::from_bits(val)
    }
}
impl From<KppRowSelectInput3Daisy> for u8 {
    #[inline(always)]
    fn from(val: KppRowSelectInput3Daisy) -> u8 {
        KppRowSelectInput3Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1HreqSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT6"]
    GPIO_AD_06_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_10 for Mode: ALT1"]
    GPIO_10_ALT1 = 0x01,
}
impl Lpi2c1HreqSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1HreqSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1HreqSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1HreqSelectInputDaisy {
        Lpi2c1HreqSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1HreqSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1HreqSelectInputDaisy) -> u8 {
        Lpi2c1HreqSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1SclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_14 for Mode: ALT0"]
    GPIO_AD_14_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_06 for Mode: ALT1"]
    GPIO_SD_06_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT1"]
    GPIO_12_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_02 for Mode: ALT3"]
    GPIO_02_ALT3 = 0x03,
}
impl Lpi2c1SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1SclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1SclSelectInputDaisy {
        Lpi2c1SclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1SclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1SclSelectInputDaisy) -> u8 {
        Lpi2c1SclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1SdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT0"]
    GPIO_AD_13_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_05 for Mode: ALT1"]
    GPIO_SD_05_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_11 for Mode: ALT1"]
    GPIO_11_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_01 for Mode: ALT3"]
    GPIO_01_ALT3 = 0x03,
}
impl Lpi2c1SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1SdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1SdaSelectInputDaisy {
        Lpi2c1SdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c1SdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1SdaSelectInputDaisy) -> u8 {
        Lpi2c1SdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2SclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT0"]
    GPIO_AD_08_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT3"]
    GPIO_AD_02_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_08 for Mode: ALT1"]
    GPIO_SD_08_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_10 for Mode: ALT3"]
    GPIO_10_ALT3 = 0x03,
}
impl Lpi2c2SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2SclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2SclSelectInputDaisy {
        Lpi2c2SclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2SclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2SclSelectInputDaisy) -> u8 {
        Lpi2c2SclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2SdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT0"]
    GPIO_AD_07_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT3"]
    GPIO_AD_01_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_SD_07 for Mode: ALT1"]
    GPIO_SD_07_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_09 for Mode: ALT3"]
    GPIO_09_ALT3 = 0x03,
}
impl Lpi2c2SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2SdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2SdaSelectInputDaisy {
        Lpi2c2SdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c2SdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2SdaSelectInputDaisy) -> u8 {
        Lpi2c2SdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1PcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_05 for Mode: ALT0"]
    GPIO_AD_05_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_07 for Mode: ALT2"]
    GPIO_SD_07_ALT2 = 0x01,
}
impl Lpspi1PcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1PcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1PcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1PcsSelectInput0Daisy {
        Lpspi1PcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi1PcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1PcsSelectInput0Daisy) -> u8 {
        Lpspi1PcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_06 for Mode: ALT0"]
    GPIO_AD_06_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_08 for Mode: ALT2"]
    GPIO_SD_08_ALT2 = 0x01,
}
impl Lpspi1SckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1SckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1SckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1SckSelectInputDaisy {
        Lpspi1SckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1SckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1SckSelectInputDaisy) -> u8 {
        Lpspi1SckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1SdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_03 for Mode: ALT0"]
    GPIO_AD_03_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_05 for Mode: ALT2"]
    GPIO_SD_05_ALT2 = 0x01,
}
impl Lpspi1SdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1SdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1SdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1SdiSelectInputDaisy {
        Lpspi1SdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1SdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1SdiSelectInputDaisy) -> u8 {
        Lpspi1SdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1SdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_04 for Mode: ALT0"]
    GPIO_AD_04_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_06 for Mode: ALT2"]
    GPIO_SD_06_ALT2 = 0x01,
}
impl Lpspi1SdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1SdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1SdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1SdoSelectInputDaisy {
        Lpspi1SdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1SdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1SdoSelectInputDaisy) -> u8 {
        Lpspi1SdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2PcsSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_11 for Mode: ALT0"]
    GPIO_AD_11_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT1"]
    GPIO_SD_12_ALT1 = 0x01,
}
impl Lpspi2PcsSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2PcsSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2PcsSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2PcsSelectInput0Daisy {
        Lpspi2PcsSelectInput0Daisy::from_bits(val)
    }
}
impl From<Lpspi2PcsSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2PcsSelectInput0Daisy) -> u8 {
        Lpspi2PcsSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_12 for Mode: ALT0"]
    GPIO_AD_12_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_11 for Mode: ALT1"]
    GPIO_SD_11_ALT1 = 0x01,
}
impl Lpspi2SckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2SckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2SckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2SckSelectInputDaisy {
        Lpspi2SckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2SckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2SckSelectInputDaisy) -> u8 {
        Lpspi2SckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2SdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_09 for Mode: ALT0"]
    GPIO_AD_09_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_09 for Mode: ALT1"]
    GPIO_SD_09_ALT1 = 0x01,
}
impl Lpspi2SdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2SdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2SdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2SdiSelectInputDaisy {
        Lpspi2SdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2SdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2SdiSelectInputDaisy) -> u8 {
        Lpspi2SdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2SdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT0"]
    GPIO_AD_10_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_10 for Mode: ALT1"]
    GPIO_SD_10_ALT1 = 0x01,
}
impl Lpspi2SdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2SdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2SdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2SdoSelectInputDaisy {
        Lpspi2SdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2SdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2SdoSelectInputDaisy) -> u8 {
        Lpspi2SdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1RxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_11 for Mode: ALT2"]
    GPIO_SD_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_09 for Mode: ALT0"]
    GPIO_09_ALT0 = 0x01,
}
impl Lpuart1RxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1RxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1RxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1RxdSelectInputDaisy {
        Lpuart1RxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart1RxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1RxdSelectInputDaisy) -> u8 {
        Lpuart1RxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1TxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_12 for Mode: ALT2"]
    GPIO_SD_12_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_10 for Mode: ALT0"]
    GPIO_10_ALT0 = 0x01,
}
impl Lpuart1TxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1TxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1TxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1TxdSelectInputDaisy {
        Lpuart1TxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart1TxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1TxdSelectInputDaisy) -> u8 {
        Lpuart1TxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2RxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_09 for Mode: ALT2"]
    GPIO_SD_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_13 for Mode: ALT0"]
    GPIO_13_ALT0 = 0x01,
}
impl Lpuart2RxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2RxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2RxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2RxdSelectInputDaisy {
        Lpuart2RxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2RxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2RxdSelectInputDaisy) -> u8 {
        Lpuart2RxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2TxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT0"]
    GPIO_AD_00_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_10 for Mode: ALT2"]
    GPIO_SD_10_ALT2 = 0x01,
}
impl Lpuart2TxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2TxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2TxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2TxdSelectInputDaisy {
        Lpuart2TxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2TxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2TxdSelectInputDaisy) -> u8 {
        Lpuart2TxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3RxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT1"]
    GPIO_AD_07_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_11 for Mode: ALT0"]
    GPIO_11_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_07 for Mode: ALT3"]
    GPIO_07_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3RxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3RxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3RxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3RxdSelectInputDaisy {
        Lpuart3RxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3RxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3RxdSelectInputDaisy) -> u8 {
        Lpuart3RxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3TxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_08 for Mode: ALT1"]
    GPIO_AD_08_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT0"]
    GPIO_12_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_08 for Mode: ALT3"]
    GPIO_08_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3TxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3TxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3TxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3TxdSelectInputDaisy {
        Lpuart3TxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3TxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3TxdSelectInputDaisy) -> u8 {
        Lpuart3TxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4RxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT0"]
    GPIO_AD_01_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_05 for Mode: ALT3"]
    GPIO_05_ALT3 = 0x01,
}
impl Lpuart4RxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4RxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4RxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4RxdSelectInputDaisy {
        Lpuart4RxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4RxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4RxdSelectInputDaisy) -> u8 {
        Lpuart4RxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4TxdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_02 for Mode: ALT0"]
    GPIO_AD_02_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_06 for Mode: ALT3"]
    GPIO_06_ALT3 = 0x01,
}
impl Lpuart4TxdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4TxdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4TxdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4TxdSelectInputDaisy {
        Lpuart4TxdSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4TxdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4TxdSelectInputDaisy) -> u8 {
        Lpuart4TxdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NmiGlueNmiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_13 for Mode: ALT6"]
    GPIO_AD_13_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_00 for Mode: ALT6"]
    GPIO_AD_00_ALT6 = 0x01,
}
impl NmiGlueNmiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NmiGlueNmiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NmiGlueNmiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NmiGlueNmiSelectInputDaisy {
        NmiGlueNmiSelectInputDaisy::from_bits(val)
    }
}
impl From<NmiGlueNmiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NmiGlueNmiSelectInputDaisy) -> u8 {
        NmiGlueNmiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl Pus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pus {
    #[inline(always)]
    fn from(val: u8) -> Pus {
        Pus::from_bits(val)
    }
}
impl From<Pus> for u8 {
    #[inline(always)]
    fn from(val: Pus) -> u8 {
        Pus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpdifIn1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_10 for Mode: ALT6"]
    GPIO_10_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_04 for Mode: ALT4"]
    GPIO_04_ALT4 = 0x01,
}
impl SpdifIn1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpdifIn1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpdifIn1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SpdifIn1SelectInputDaisy {
        SpdifIn1SelectInputDaisy::from_bits(val)
    }
}
impl From<SpdifIn1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SpdifIn1SelectInputDaisy) -> u8 {
        SpdifIn1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpdifTxClk2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT6"]
    GPIO_12_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_06 for Mode: ALT4"]
    GPIO_06_ALT4 = 0x01,
}
impl SpdifTxClk2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpdifTxClk2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpdifTxClk2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SpdifTxClk2SelectInputDaisy {
        SpdifTxClk2SelectInputDaisy::from_bits(val)
    }
}
impl From<SpdifTxClk2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SpdifTxClk2SelectInputDaisy) -> u8 {
        SpdifTxClk2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Speed {
    #[doc = "low(50MHz)"]
    SPEED_0_LOW_50MHZ = 0x0,
    #[doc = "medium(100MHz)"]
    SPEED_1_MEDIUM_100MHZ = 0x01,
    #[doc = "fast(150MHz)"]
    SPEED_2_FAST_150MHZ = 0x02,
    #[doc = "max(200MHz)"]
    SPEED_3_MAX_200MHZ = 0x03,
}
impl Speed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Speed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Speed {
    #[inline(always)]
    fn from(val: u8) -> Speed {
        Speed::from_bits(val)
    }
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(val: Speed) -> u8 {
        Speed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbOtgIdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_10 for Mode: ALT6"]
    GPIO_AD_10_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_13 for Mode: ALT3"]
    GPIO_13_ALT3 = 0x01,
}
impl UsbOtgIdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbOtgIdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbOtgIdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbOtgIdSelectInputDaisy {
        UsbOtgIdSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbOtgIdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbOtgIdSelectInputDaisy) -> u8 {
        UsbOtgIdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbOtgOcSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_01 for Mode: ALT6"]
    GPIO_AD_01_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_12 for Mode: ALT3"]
    GPIO_12_ALT3 = 0x01,
}
impl UsbOtgOcSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbOtgOcSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbOtgOcSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbOtgOcSelectInputDaisy {
        UsbOtgOcSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbOtgOcSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbOtgOcSelectInputDaisy) -> u8 {
        UsbOtgOcSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XevGlueRxevSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_07 for Mode: ALT2"]
    GPIO_AD_07_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_00 for Mode: ALT2"]
    GPIO_SD_00_ALT2 = 0x01,
}
impl XevGlueRxevSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XevGlueRxevSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XevGlueRxevSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> XevGlueRxevSelectInputDaisy {
        XevGlueRxevSelectInputDaisy::from_bits(val)
    }
}
impl From<XevGlueRxevSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: XevGlueRxevSelectInputDaisy) -> u8 {
        XevGlueRxevSelectInputDaisy::to_bits(val)
    }
}
