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
