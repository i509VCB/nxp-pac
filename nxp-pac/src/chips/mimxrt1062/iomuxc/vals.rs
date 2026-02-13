#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnatopUsbOtg1IdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_01 for Mode: ALT3"]
    GPIO_AD_B0_01_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT0"]
    GPIO_AD_B1_02_ALT0 = 0x01,
}
impl AnatopUsbOtg1IdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnatopUsbOtg1IdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnatopUsbOtg1IdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> AnatopUsbOtg1IdSelectInputDaisy {
        AnatopUsbOtg1IdSelectInputDaisy::from_bits(val)
    }
}
impl From<AnatopUsbOtg1IdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: AnatopUsbOtg1IdSelectInputDaisy) -> u8 {
        AnatopUsbOtg1IdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnatopUsbOtg2IdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_00 for Mode: ALT3"]
    GPIO_AD_B0_00_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_00 for Mode: ALT0"]
    GPIO_AD_B1_00_ALT0 = 0x01,
}
impl AnatopUsbOtg2IdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnatopUsbOtg2IdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnatopUsbOtg2IdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> AnatopUsbOtg2IdSelectInputDaisy {
        AnatopUsbOtg2IdSelectInputDaisy::from_bits(val)
    }
}
impl From<AnatopUsbOtg2IdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: AnatopUsbOtg2IdSelectInputDaisy) -> u8 {
        AnatopUsbOtg2IdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanfdIppIndCanrxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT9"]
    GPIO_EMC_37_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT8"]
    GPIO_AD_B0_15_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT8"]
    GPIO_AD_B0_11_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CanfdIppIndCanrxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanfdIppIndCanrxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanfdIppIndCanrxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CanfdIppIndCanrxSelectInputDaisy {
        CanfdIppIndCanrxSelectInputDaisy::from_bits(val)
    }
}
impl From<CanfdIppIndCanrxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CanfdIppIndCanrxSelectInputDaisy) -> u8 {
        CanfdIppIndCanrxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcmPmicReadySelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT6"]
    GPIO_SD_B1_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_12 for Mode: ALT1"]
    GPIO_AD_B0_12_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT4"]
    GPIO_AD_B1_01_ALT4 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B1_08 for Mode: ALT3"]
    GPIO_AD_B1_08_ALT3 = 0x03,
    #[doc = "Selecting Pad: GPIO_EMC_32 for Mode: ALT3"]
    GPIO_EMC_32_ALT3 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl CcmPmicReadySelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcmPmicReadySelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcmPmicReadySelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CcmPmicReadySelectInputDaisy {
        CcmPmicReadySelectInputDaisy::from_bits(val)
    }
}
impl From<CcmPmicReadySelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CcmPmicReadySelectInputDaisy) -> u8 {
        CcmPmicReadySelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData02SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT4"]
    GPIO_AD_B1_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT4"]
    GPIO_AD_B0_11_ALT4 = 0x01,
}
impl CsiData02SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData02SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData02SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData02SelectInputDaisy {
        CsiData02SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData02SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData02SelectInputDaisy) -> u8 {
        CsiData02SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData03SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT4"]
    GPIO_AD_B1_14_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT4"]
    GPIO_AD_B0_10_ALT4 = 0x01,
}
impl CsiData03SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData03SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData03SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData03SelectInputDaisy {
        CsiData03SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData03SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData03SelectInputDaisy) -> u8 {
        CsiData03SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData04SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_13 for Mode: ALT4"]
    GPIO_AD_B1_13_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT4"]
    GPIO_AD_B0_09_ALT4 = 0x01,
}
impl CsiData04SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData04SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData04SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData04SelectInputDaisy {
        CsiData04SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData04SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData04SelectInputDaisy) -> u8 {
        CsiData04SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData05SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT4"]
    GPIO_AD_B1_12_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_08 for Mode: ALT4"]
    GPIO_AD_B0_08_ALT4 = 0x01,
}
impl CsiData05SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData05SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData05SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData05SelectInputDaisy {
        CsiData05SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData05SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData05SelectInputDaisy) -> u8 {
        CsiData05SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData06SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT4"]
    GPIO_AD_B1_11_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_07 for Mode: ALT4"]
    GPIO_AD_B0_07_ALT4 = 0x01,
}
impl CsiData06SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData06SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData06SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData06SelectInputDaisy {
        CsiData06SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData06SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData06SelectInputDaisy) -> u8 {
        CsiData06SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData07SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT4"]
    GPIO_AD_B1_10_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_06 for Mode: ALT4"]
    GPIO_AD_B0_06_ALT4 = 0x01,
}
impl CsiData07SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData07SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData07SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData07SelectInputDaisy {
        CsiData07SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData07SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData07SelectInputDaisy) -> u8 {
        CsiData07SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData08SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT4"]
    GPIO_AD_B1_09_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT4"]
    GPIO_AD_B0_05_ALT4 = 0x01,
}
impl CsiData08SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData08SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData08SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData08SelectInputDaisy {
        CsiData08SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData08SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData08SelectInputDaisy) -> u8 {
        CsiData08SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiData09SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_08 for Mode: ALT4"]
    GPIO_AD_B1_08_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_04 for Mode: ALT4"]
    GPIO_AD_B0_04_ALT4 = 0x01,
}
impl CsiData09SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiData09SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiData09SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiData09SelectInputDaisy {
        CsiData09SelectInputDaisy::from_bits(val)
    }
}
impl From<CsiData09SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiData09SelectInputDaisy) -> u8 {
        CsiData09SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiHsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT4"]
    GPIO_AD_B0_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT4"]
    GPIO_AD_B1_07_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_14 for Mode: ALT2"]
    GPIO_B1_14_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CsiHsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiHsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiHsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiHsyncSelectInputDaisy {
        CsiHsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<CsiHsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiHsyncSelectInputDaisy) -> u8 {
        CsiHsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiPixclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_04 for Mode: ALT4"]
    GPIO_AD_B1_04_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT2"]
    GPIO_B1_12_ALT2 = 0x01,
}
impl CsiPixclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiPixclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiPixclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiPixclkSelectInputDaisy {
        CsiPixclkSelectInputDaisy::from_bits(val)
    }
}
impl From<CsiPixclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiPixclkSelectInputDaisy) -> u8 {
        CsiPixclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsiVsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_14 for Mode: ALT4"]
    GPIO_AD_B0_14_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT4"]
    GPIO_AD_B1_06_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT2"]
    GPIO_B1_13_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CsiVsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsiVsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsiVsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> CsiVsyncSelectInputDaisy {
        CsiVsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<CsiVsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: CsiVsyncSelectInputDaisy) -> u8 {
        CsiVsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM_3_3V_260_OHM_1_8V = 0x01,
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
pub enum Enet0RxdataSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT3"]
    GPIO_EMC_20_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT3"]
    GPIO_B1_04_ALT3 = 0x01,
}
impl Enet0RxdataSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet0RxdataSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet0RxdataSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet0RxdataSelectInputDaisy {
        Enet0RxdataSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet0RxdataSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet0RxdataSelectInputDaisy) -> u8 {
        Enet0RxdataSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet0TimerSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT3"]
    GPIO_AD_B0_15_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT7"]
    GPIO_AD_B0_11_ALT7 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT3"]
    GPIO_B1_12_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet0TimerSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet0TimerSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet0TimerSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet0TimerSelectInputDaisy {
        Enet0TimerSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet0TimerSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet0TimerSelectInputDaisy) -> u8 {
        Enet0TimerSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet1RxdataSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT3"]
    GPIO_EMC_19_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT3"]
    GPIO_B1_05_ALT3 = 0x01,
}
impl Enet1RxdataSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet1RxdataSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet1RxdataSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet1RxdataSelectInputDaisy {
        Enet1RxdataSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet1RxdataSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet1RxdataSelectInputDaisy) -> u8 {
        Enet1RxdataSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IpgClkRmiiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT9"]
    GPIO_EMC_33_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT9"]
    GPIO_SD_B0_01_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT9"]
    GPIO_B0_15_ALT9 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IpgClkRmiiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IpgClkRmiiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IpgClkRmiiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IpgClkRmiiSelectInputDaisy {
        Enet2IpgClkRmiiSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2IpgClkRmiiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IpgClkRmiiSelectInputDaisy) -> u8 {
        Enet2IpgClkRmiiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0MdioSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT8"]
    GPIO_EMC_39_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_01 for Mode: ALT8"]
    GPIO_B0_01_ALT8 = 0x01,
}
impl Enet2IppIndMac0MdioSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0MdioSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0MdioSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0MdioSelectInputDaisy {
        Enet2IppIndMac0MdioSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0MdioSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0MdioSelectInputDaisy) -> u8 {
        Enet2IppIndMac0MdioSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0RxdataSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT8"]
    GPIO_EMC_35_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_03 for Mode: ALT8"]
    GPIO_SD_B0_03_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT8"]
    GPIO_B1_01_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IppIndMac0RxdataSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0RxdataSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0RxdataSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0RxdataSelectInput0Daisy {
        Enet2IppIndMac0RxdataSelectInput0Daisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0RxdataSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0RxdataSelectInput0Daisy) -> u8 {
        Enet2IppIndMac0RxdataSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0RxdataSelectInput1Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_36 for Mode: ALT8"]
    GPIO_EMC_36_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT8"]
    GPIO_SD_B0_04_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT8"]
    GPIO_B1_02_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IppIndMac0RxdataSelectInput1Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0RxdataSelectInput1Daisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0RxdataSelectInput1Daisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0RxdataSelectInput1Daisy {
        Enet2IppIndMac0RxdataSelectInput1Daisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0RxdataSelectInput1Daisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0RxdataSelectInput1Daisy) -> u8 {
        Enet2IppIndMac0RxdataSelectInput1Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0RxenSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT8"]
    GPIO_EMC_37_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT8"]
    GPIO_SD_B0_05_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT8"]
    GPIO_B1_03_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IppIndMac0RxenSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0RxenSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0RxenSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0RxenSelectInputDaisy {
        Enet2IppIndMac0RxenSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0RxenSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0RxenSelectInputDaisy) -> u8 {
        Enet2IppIndMac0RxenSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0RxerrSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_34 for Mode: ALT8"]
    GPIO_EMC_34_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_02 for Mode: ALT8"]
    GPIO_SD_B0_02_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT8"]
    GPIO_B1_00_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IppIndMac0RxerrSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0RxerrSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0RxerrSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0RxerrSelectInputDaisy {
        Enet2IppIndMac0RxerrSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0RxerrSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0RxerrSelectInputDaisy) -> u8 {
        Enet2IppIndMac0RxerrSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0TimerSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT8"]
    GPIO_AD_B1_01_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_03 for Mode: ALT8"]
    GPIO_B0_03_ALT8 = 0x01,
}
impl Enet2IppIndMac0TimerSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0TimerSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0TimerSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0TimerSelectInput0Daisy {
        Enet2IppIndMac0TimerSelectInput0Daisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0TimerSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0TimerSelectInput0Daisy) -> u8 {
        Enet2IppIndMac0TimerSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2IppIndMac0TxclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT8"]
    GPIO_EMC_33_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT8"]
    GPIO_SD_B0_01_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT8"]
    GPIO_B0_15_ALT8 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enet2IppIndMac0TxclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2IppIndMac0TxclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2IppIndMac0TxclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Enet2IppIndMac0TxclkSelectInputDaisy {
        Enet2IppIndMac0TxclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Enet2IppIndMac0TxclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Enet2IppIndMac0TxclkSelectInputDaisy) -> u8 {
        Enet2IppIndMac0TxclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetIpgClkRmiiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT4"]
    GPIO_EMC_25_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT6"]
    GPIO_B1_10_ALT6 = 0x01,
}
impl EnetIpgClkRmiiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetIpgClkRmiiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetIpgClkRmiiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetIpgClkRmiiSelectInputDaisy {
        EnetIpgClkRmiiSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetIpgClkRmiiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetIpgClkRmiiSelectInputDaisy) -> u8 {
        EnetIpgClkRmiiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetMdioSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_05 for Mode: ALT1"]
    GPIO_AD_B1_05_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_41 for Mode: ALT4"]
    GPIO_EMC_41_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_15 for Mode: ALT0"]
    GPIO_B1_15_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl EnetMdioSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetMdioSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetMdioSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetMdioSelectInputDaisy {
        EnetMdioSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetMdioSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetMdioSelectInputDaisy) -> u8 {
        EnetMdioSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetRxenSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT3"]
    GPIO_EMC_23_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT3"]
    GPIO_B1_06_ALT3 = 0x01,
}
impl EnetRxenSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetRxenSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetRxenSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetRxenSelectInputDaisy {
        EnetRxenSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetRxenSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetRxenSelectInputDaisy) -> u8 {
        EnetRxenSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetRxerrSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT3"]
    GPIO_EMC_26_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_11 for Mode: ALT3"]
    GPIO_B1_11_ALT3 = 0x01,
}
impl EnetRxerrSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetRxerrSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetRxerrSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetRxerrSelectInputDaisy {
        EnetRxerrSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetRxerrSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetRxerrSelectInputDaisy) -> u8 {
        EnetRxerrSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetTxclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT3"]
    GPIO_EMC_25_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT3"]
    GPIO_B1_10_ALT3 = 0x01,
}
impl EnetTxclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetTxclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetTxclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> EnetTxclkSelectInputDaisy {
        EnetTxclkSelectInputDaisy::from_bits(val)
    }
}
impl From<EnetTxclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: EnetTxclkSelectInputDaisy) -> u8 {
        EnetTxclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT4"]
    GPIO_SD_B1_03_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT3"]
    GPIO_EMC_18_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT2"]
    GPIO_AD_B1_09_ALT2 = 0x02,
    #[doc = "Selecting Pad: GPIO_B0_03 for Mode: ALT2"]
    GPIO_B0_03_ALT2 = 0x03,
}
impl Flexcan1RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1RxSelectInputDaisy {
        Flexcan1RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexcan1RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1RxSelectInputDaisy) -> u8 {
        Flexcan1RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan2RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT3"]
    GPIO_EMC_10_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT0"]
    GPIO_AD_B0_03_ALT0 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT6"]
    GPIO_AD_B0_15_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT6"]
    GPIO_B1_09_ALT6 = 0x03,
}
impl Flexcan2RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan2RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan2RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexcan2RxSelectInputDaisy {
        Flexcan2RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexcan2RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexcan2RxSelectInputDaisy) -> u8 {
        Flexcan2RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwma0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT1"]
    GPIO_EMC_23_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT1"]
    GPIO_SD_B0_00_ALT1 = 0x01,
}
impl Flexpwm1Pwma0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwma0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwma0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwma0SelectInputDaisy {
        Flexpwm1Pwma0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwma0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwma0SelectInputDaisy) -> u8 {
        Flexpwm1Pwma0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwma1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT1"]
    GPIO_EMC_25_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_02 for Mode: ALT1"]
    GPIO_SD_B0_02_ALT1 = 0x01,
}
impl Flexpwm1Pwma1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwma1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwma1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwma1SelectInputDaisy {
        Flexpwm1Pwma1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwma1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwma1SelectInputDaisy) -> u8 {
        Flexpwm1Pwma1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwma2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT1"]
    GPIO_EMC_27_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT1"]
    GPIO_SD_B0_04_ALT1 = 0x01,
}
impl Flexpwm1Pwma2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwma2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwma2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwma2SelectInputDaisy {
        Flexpwm1Pwma2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwma2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwma2SelectInputDaisy) -> u8 {
        Flexpwm1Pwma2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwma3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT2"]
    GPIO_SD_B1_00_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT4"]
    GPIO_EMC_12_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_38 for Mode: ALT1"]
    GPIO_EMC_38_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT1"]
    GPIO_AD_B0_10_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT6"]
    GPIO_B1_00_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm1Pwma3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwma3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwma3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwma3SelectInputDaisy {
        Flexpwm1Pwma3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwma3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwma3SelectInputDaisy) -> u8 {
        Flexpwm1Pwma3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwmb0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT1"]
    GPIO_EMC_24_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT1"]
    GPIO_SD_B0_01_ALT1 = 0x01,
}
impl Flexpwm1Pwmb0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwmb0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwmb0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwmb0SelectInputDaisy {
        Flexpwm1Pwmb0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwmb0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwmb0SelectInputDaisy) -> u8 {
        Flexpwm1Pwmb0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwmb1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT1"]
    GPIO_EMC_26_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_03 for Mode: ALT1"]
    GPIO_SD_B0_03_ALT1 = 0x01,
}
impl Flexpwm1Pwmb1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwmb1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwmb1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwmb1SelectInputDaisy {
        Flexpwm1Pwmb1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwmb1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwmb1SelectInputDaisy) -> u8 {
        Flexpwm1Pwmb1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwmb2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_28 for Mode: ALT1"]
    GPIO_EMC_28_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT1"]
    GPIO_SD_B0_05_ALT1 = 0x01,
}
impl Flexpwm1Pwmb2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwmb2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwmb2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwmb2SelectInputDaisy {
        Flexpwm1Pwmb2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwmb2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwmb2SelectInputDaisy) -> u8 {
        Flexpwm1Pwmb2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm1Pwmb3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT2"]
    GPIO_SD_B1_01_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT4"]
    GPIO_EMC_13_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT1"]
    GPIO_EMC_39_ALT1 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT1"]
    GPIO_AD_B0_11_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT6"]
    GPIO_B1_01_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm1Pwmb3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm1Pwmb3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm1Pwmb3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm1Pwmb3SelectInputDaisy {
        Flexpwm1Pwmb3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm1Pwmb3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm1Pwmb3SelectInputDaisy) -> u8 {
        Flexpwm1Pwmb3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwma0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_06 for Mode: ALT1"]
    GPIO_EMC_06_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_06 for Mode: ALT2"]
    GPIO_B0_06_ALT2 = 0x01,
}
impl Flexpwm2Pwma0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwma0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwma0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwma0SelectInputDaisy {
        Flexpwm2Pwma0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwma0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwma0SelectInputDaisy) -> u8 {
        Flexpwm2Pwma0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwma1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT1"]
    GPIO_EMC_08_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_08 for Mode: ALT2"]
    GPIO_B0_08_ALT2 = 0x01,
}
impl Flexpwm2Pwma1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwma1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwma1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwma1SelectInputDaisy {
        Flexpwm2Pwma1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwma1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwma1SelectInputDaisy) -> u8 {
        Flexpwm2Pwma1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwma2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT1"]
    GPIO_EMC_10_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_10 for Mode: ALT2"]
    GPIO_B0_10_ALT2 = 0x01,
}
impl Flexpwm2Pwma2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwma2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwma2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwma2SelectInputDaisy {
        Flexpwm2Pwma2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwma2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwma2SelectInputDaisy) -> u8 {
        Flexpwm2Pwma2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwma3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT2"]
    GPIO_SD_B1_02_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT1"]
    GPIO_EMC_19_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_00 for Mode: ALT0"]
    GPIO_AD_B0_00_ALT0 = 0x02,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT1"]
    GPIO_AD_B0_09_ALT1 = 0x03,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT6"]
    GPIO_B1_02_ALT6 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flexpwm2Pwma3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwma3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwma3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwma3SelectInputDaisy {
        Flexpwm2Pwma3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwma3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwma3SelectInputDaisy) -> u8 {
        Flexpwm2Pwma3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwmb0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_07 for Mode: ALT1"]
    GPIO_EMC_07_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_07 for Mode: ALT2"]
    GPIO_B0_07_ALT2 = 0x01,
}
impl Flexpwm2Pwmb0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwmb0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwmb0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwmb0SelectInputDaisy {
        Flexpwm2Pwmb0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwmb0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwmb0SelectInputDaisy) -> u8 {
        Flexpwm2Pwmb0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwmb1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_09 for Mode: ALT1"]
    GPIO_EMC_09_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_09 for Mode: ALT2"]
    GPIO_B0_09_ALT2 = 0x01,
}
impl Flexpwm2Pwmb1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwmb1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwmb1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwmb1SelectInputDaisy {
        Flexpwm2Pwmb1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwmb1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwmb1SelectInputDaisy) -> u8 {
        Flexpwm2Pwmb1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwmb2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_11 for Mode: ALT1"]
    GPIO_EMC_11_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_11 for Mode: ALT2"]
    GPIO_B0_11_ALT2 = 0x01,
}
impl Flexpwm2Pwmb2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwmb2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwmb2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwmb2SelectInputDaisy {
        Flexpwm2Pwmb2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwmb2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwmb2SelectInputDaisy) -> u8 {
        Flexpwm2Pwmb2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm2Pwmb3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT2"]
    GPIO_SD_B1_03_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT1"]
    GPIO_EMC_20_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_01 for Mode: ALT0"]
    GPIO_AD_B0_01_ALT0 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT6"]
    GPIO_B1_03_ALT6 = 0x03,
}
impl Flexpwm2Pwmb3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm2Pwmb3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm2Pwmb3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm2Pwmb3SelectInputDaisy {
        Flexpwm2Pwmb3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm2Pwmb3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm2Pwmb3SelectInputDaisy) -> u8 {
        Flexpwm2Pwmb3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm4Pwma0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_00 for Mode: ALT1"]
    GPIO_EMC_00_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_08 for Mode: ALT1"]
    GPIO_AD_B1_08_ALT1 = 0x01,
}
impl Flexpwm4Pwma0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm4Pwma0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm4Pwma0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm4Pwma0SelectInputDaisy {
        Flexpwm4Pwma0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm4Pwma0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm4Pwma0SelectInputDaisy) -> u8 {
        Flexpwm4Pwma0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm4Pwma1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_02 for Mode: ALT1"]
    GPIO_EMC_02_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT1"]
    GPIO_AD_B1_09_ALT1 = 0x01,
}
impl Flexpwm4Pwma1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm4Pwma1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm4Pwma1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm4Pwma1SelectInputDaisy {
        Flexpwm4Pwma1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm4Pwma1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm4Pwma1SelectInputDaisy) -> u8 {
        Flexpwm4Pwma1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm4Pwma2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_04 for Mode: ALT1"]
    GPIO_EMC_04_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_14 for Mode: ALT1"]
    GPIO_B1_14_ALT1 = 0x01,
}
impl Flexpwm4Pwma2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm4Pwma2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm4Pwma2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm4Pwma2SelectInputDaisy {
        Flexpwm4Pwma2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm4Pwma2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm4Pwma2SelectInputDaisy) -> u8 {
        Flexpwm4Pwma2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexpwm4Pwma3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_17 for Mode: ALT1"]
    GPIO_EMC_17_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_15 for Mode: ALT1"]
    GPIO_B1_15_ALT1 = 0x01,
}
impl Flexpwm4Pwma3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexpwm4Pwma3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexpwm4Pwma3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexpwm4Pwma3SelectInputDaisy {
        Flexpwm4Pwma3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexpwm4Pwma3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexpwm4Pwma3SelectInputDaisy) -> u8 {
        Flexpwm4Pwma3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndDqsFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_00 for Mode: ALT0"]
    GPIO_SPI_B1_00_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT8"]
    GPIO_EMC_23_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_09 for Mode: ALT0"]
    GPIO_SPI_B0_09_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndDqsFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndDqsFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndDqsFaSelectInputDaisy {
        Flexspi2IppIndDqsFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndDqsFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndDqsFaSelectInputDaisy) -> u8 {
        Flexspi2IppIndDqsFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFaBit0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_04 for Mode: ALT0"]
    GPIO_SPI_B1_04_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT8"]
    GPIO_EMC_26_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_02 for Mode: ALT0"]
    GPIO_SPI_B0_02_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndIoFaBit0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFaBit0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFaBit0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFaBit0SelectInputDaisy {
        Flexspi2IppIndIoFaBit0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFaBit0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFaBit0SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFaBit0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFaBit1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_03 for Mode: ALT0"]
    GPIO_SPI_B1_03_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT8"]
    GPIO_EMC_27_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_12 for Mode: ALT0"]
    GPIO_SPI_B0_12_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndIoFaBit1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFaBit1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFaBit1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFaBit1SelectInputDaisy {
        Flexspi2IppIndIoFaBit1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFaBit1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFaBit1SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFaBit1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFaBit2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_02 for Mode: ALT0"]
    GPIO_SPI_B1_02_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_28 for Mode: ALT8"]
    GPIO_EMC_28_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_06 for Mode: ALT0"]
    GPIO_SPI_B0_06_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndIoFaBit2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFaBit2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFaBit2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFaBit2SelectInputDaisy {
        Flexspi2IppIndIoFaBit2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFaBit2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFaBit2SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFaBit2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFaBit3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_01 for Mode: ALT0"]
    GPIO_SPI_B1_01_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_29 for Mode: ALT8"]
    GPIO_EMC_29_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_10 for Mode: ALT0"]
    GPIO_SPI_B0_10_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndIoFaBit3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFaBit3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFaBit3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFaBit3SelectInputDaisy {
        Flexspi2IppIndIoFaBit3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFaBit3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFaBit3SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFaBit3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFbBit0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT8"]
    GPIO_EMC_13_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SPI_B0_11 for Mode: ALT0"]
    GPIO_SPI_B0_11_ALT0 = 0x01,
}
impl Flexspi2IppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFbBit0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFbBit0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFbBit0SelectInputDaisy {
        Flexspi2IppIndIoFbBit0SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFbBit0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFbBit0SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFbBit0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFbBit1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_14 for Mode: ALT8"]
    GPIO_EMC_14_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SPI_B0_07 for Mode: ALT0"]
    GPIO_SPI_B0_07_ALT0 = 0x01,
}
impl Flexspi2IppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFbBit1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFbBit1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFbBit1SelectInputDaisy {
        Flexspi2IppIndIoFbBit1SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFbBit1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFbBit1SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFbBit1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFbBit2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT8"]
    GPIO_EMC_15_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SPI_B0_03 for Mode: ALT0"]
    GPIO_SPI_B0_03_ALT0 = 0x01,
}
impl Flexspi2IppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFbBit2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFbBit2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFbBit2SelectInputDaisy {
        Flexspi2IppIndIoFbBit2SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFbBit2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFbBit2SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFbBit2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndIoFbBit3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT8"]
    GPIO_EMC_16_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SPI_B0_04 for Mode: ALT0"]
    GPIO_SPI_B0_04_ALT0 = 0x01,
}
impl Flexspi2IppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndIoFbBit3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndIoFbBit3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndIoFbBit3SelectInputDaisy {
        Flexspi2IppIndIoFbBit3SelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndIoFbBit3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndIoFbBit3SelectInputDaisy) -> u8 {
        Flexspi2IppIndIoFbBit3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndSckFaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SPI_B1_05 for Mode: ALT0"]
    GPIO_SPI_B1_05_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT8"]
    GPIO_EMC_25_ALT8 = 0x01,
    #[doc = "Selecting Pad: GPIO_SPI_B0_08 for Mode: ALT0"]
    GPIO_SPI_B0_08_ALT0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Flexspi2IppIndSckFaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndSckFaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndSckFaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndSckFaSelectInputDaisy {
        Flexspi2IppIndSckFaSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndSckFaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndSckFaSelectInputDaisy) -> u8 {
        Flexspi2IppIndSckFaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2IppIndSckFbSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT8"]
    GPIO_EMC_12_ALT8 = 0x0,
    #[doc = "Selecting Pad: GPIO_SPI_B0_01 for Mode: ALT0"]
    GPIO_SPI_B0_01_ALT0 = 0x01,
}
impl Flexspi2IppIndSckFbSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2IppIndSckFbSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2IppIndSckFbSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2IppIndSckFbSelectInputDaisy {
        Flexspi2IppIndSckFbSelectInputDaisy::from_bits(val)
    }
}
impl From<Flexspi2IppIndSckFbSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2IppIndSckFbSelectInputDaisy) -> u8 {
        Flexspi2IppIndSckFbSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaData0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT1"]
    GPIO_SD_B1_08_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_13 for Mode: ALT0"]
    GPIO_AD_B1_13_ALT0 = 0x01,
}
impl FlexspiaData0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaData0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaData0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaData0SelectInputDaisy {
        FlexspiaData0SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaData0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaData0SelectInputDaisy) -> u8 {
        FlexspiaData0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaData1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT1"]
    GPIO_SD_B1_09_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT0"]
    GPIO_AD_B1_12_ALT0 = 0x01,
}
impl FlexspiaData1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaData1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaData1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaData1SelectInputDaisy {
        FlexspiaData1SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaData1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaData1SelectInputDaisy) -> u8 {
        FlexspiaData1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaData2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT1"]
    GPIO_SD_B1_10_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT0"]
    GPIO_AD_B1_11_ALT0 = 0x01,
}
impl FlexspiaData2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaData2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaData2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaData2SelectInputDaisy {
        FlexspiaData2SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaData2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaData2SelectInputDaisy) -> u8 {
        FlexspiaData2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaData3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_11 for Mode: ALT1"]
    GPIO_SD_B1_11_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT0"]
    GPIO_AD_B1_10_ALT0 = 0x01,
}
impl FlexspiaData3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaData3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaData3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaData3SelectInputDaisy {
        FlexspiaData3SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaData3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaData3SelectInputDaisy) -> u8 {
        FlexspiaData3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaDqsSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT1"]
    GPIO_SD_B1_05_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT0"]
    GPIO_AD_B1_09_ALT0 = 0x01,
}
impl FlexspiaDqsSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaDqsSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaDqsSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaDqsSelectInputDaisy {
        FlexspiaDqsSelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaDqsSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaDqsSelectInputDaisy) -> u8 {
        FlexspiaDqsSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiaSckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT1"]
    GPIO_SD_B1_07_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT0"]
    GPIO_AD_B1_14_ALT0 = 0x01,
}
impl FlexspiaSckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiaSckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiaSckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiaSckSelectInputDaisy {
        FlexspiaSckSelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspiaSckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiaSckSelectInputDaisy) -> u8 {
        FlexspiaSckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspibData0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT1"]
    GPIO_SD_B1_03_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT0"]
    GPIO_AD_B1_07_ALT0 = 0x01,
}
impl FlexspibData0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspibData0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspibData0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspibData0SelectInputDaisy {
        FlexspibData0SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspibData0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspibData0SelectInputDaisy) -> u8 {
        FlexspibData0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspibData1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT1"]
    GPIO_SD_B1_02_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT0"]
    GPIO_AD_B1_06_ALT0 = 0x01,
}
impl FlexspibData1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspibData1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspibData1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspibData1SelectInputDaisy {
        FlexspibData1SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspibData1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspibData1SelectInputDaisy) -> u8 {
        FlexspibData1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspibData2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT1"]
    GPIO_SD_B1_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_05 for Mode: ALT0"]
    GPIO_AD_B1_05_ALT0 = 0x01,
}
impl FlexspibData2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspibData2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspibData2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspibData2SelectInputDaisy {
        FlexspibData2SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspibData2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspibData2SelectInputDaisy) -> u8 {
        FlexspibData2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspibData3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT1"]
    GPIO_SD_B1_00_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_04 for Mode: ALT0"]
    GPIO_AD_B1_04_ALT0 = 0x01,
}
impl FlexspibData3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspibData3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspibData3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> FlexspibData3SelectInputDaisy {
        FlexspibData3SelectInputDaisy::from_bits(val)
    }
}
impl From<FlexspibData3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: FlexspibData3SelectInputDaisy) -> u8 {
        FlexspibData3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt1IppIndCapin1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT4"]
    GPIO_EMC_24_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT8"]
    GPIO_B1_05_ALT8 = 0x01,
}
impl Gpt1IppIndCapin1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt1IppIndCapin1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt1IppIndCapin1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt1IppIndCapin1SelectInputDaisy {
        Gpt1IppIndCapin1SelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt1IppIndCapin1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt1IppIndCapin1SelectInputDaisy) -> u8 {
        Gpt1IppIndCapin1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt1IppIndCapin2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT4"]
    GPIO_EMC_23_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT8"]
    GPIO_B1_06_ALT8 = 0x01,
}
impl Gpt1IppIndCapin2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt1IppIndCapin2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt1IppIndCapin2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt1IppIndCapin2SelectInputDaisy {
        Gpt1IppIndCapin2SelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt1IppIndCapin2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt1IppIndCapin2SelectInputDaisy) -> u8 {
        Gpt1IppIndCapin2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt1IppIndClkinSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_13 for Mode: ALT1"]
    GPIO_AD_B0_13_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_04 for Mode: ALT8"]
    GPIO_B1_04_ALT8 = 0x01,
}
impl Gpt1IppIndClkinSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt1IppIndClkinSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt1IppIndClkinSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt1IppIndClkinSelectInputDaisy {
        Gpt1IppIndClkinSelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt1IppIndClkinSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt1IppIndClkinSelectInputDaisy) -> u8 {
        Gpt1IppIndClkinSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt2IppIndCapin1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_41 for Mode: ALT1"]
    GPIO_EMC_41_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT8"]
    GPIO_AD_B1_03_ALT8 = 0x01,
}
impl Gpt2IppIndCapin1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt2IppIndCapin1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt2IppIndCapin1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt2IppIndCapin1SelectInputDaisy {
        Gpt2IppIndCapin1SelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt2IppIndCapin1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt2IppIndCapin1SelectInputDaisy) -> u8 {
        Gpt2IppIndCapin1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt2IppIndCapin2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_40 for Mode: ALT1"]
    GPIO_EMC_40_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_04 for Mode: ALT8"]
    GPIO_AD_B1_04_ALT8 = 0x01,
}
impl Gpt2IppIndCapin2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt2IppIndCapin2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt2IppIndCapin2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt2IppIndCapin2SelectInputDaisy {
        Gpt2IppIndCapin2SelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt2IppIndCapin2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt2IppIndCapin2SelectInputDaisy) -> u8 {
        Gpt2IppIndCapin2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt2IppIndClkinSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT7"]
    GPIO_AD_B0_09_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT8"]
    GPIO_AD_B1_02_ALT8 = 0x01,
}
impl Gpt2IppIndClkinSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt2IppIndClkinSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt2IppIndClkinSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Gpt2IppIndClkinSelectInputDaisy {
        Gpt2IppIndClkinSelectInputDaisy::from_bits(val)
    }
}
impl From<Gpt2IppIndClkinSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Gpt2IppIndClkinSelectInputDaisy) -> u8 {
        Gpt2IppIndClkinSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1SclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT2"]
    GPIO_SD_B1_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_00 for Mode: ALT3"]
    GPIO_AD_B1_00_ALT3 = 0x01,
}
impl Lpi2c1SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
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
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT2"]
    GPIO_SD_B1_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT3"]
    GPIO_AD_B1_01_ALT3 = 0x01,
}
impl Lpi2c1SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
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
    #[doc = "Selecting Pad: GPIO_SD_B1_11 for Mode: ALT3"]
    GPIO_SD_B1_11_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_04 for Mode: ALT2"]
    GPIO_B0_04_ALT2 = 0x01,
}
impl Lpi2c2SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
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
    #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT3"]
    GPIO_SD_B1_10_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_05 for Mode: ALT2"]
    GPIO_B0_05_ALT2 = 0x01,
}
impl Lpi2c2SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
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
pub enum Lpi2c3SclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT2"]
    GPIO_EMC_22_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT2"]
    GPIO_SD_B0_00_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT1"]
    GPIO_AD_B1_07_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3SclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3SclSelectInputDaisy {
        Lpi2c3SclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3SclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3SclSelectInputDaisy) -> u8 {
        Lpi2c3SclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3SdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_21 for Mode: ALT2"]
    GPIO_EMC_21_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT2"]
    GPIO_SD_B0_01_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT1"]
    GPIO_AD_B1_06_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpi2c3SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3SdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3SdaSelectInputDaisy {
        Lpi2c3SdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c3SdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3SdaSelectInputDaisy) -> u8 {
        Lpi2c3SdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4SclSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT2"]
    GPIO_EMC_12_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_12 for Mode: ALT0"]
    GPIO_AD_B0_12_ALT0 = 0x01,
}
impl Lpi2c4SclSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4SclSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4SclSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4SclSelectInputDaisy {
        Lpi2c4SclSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c4SclSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4SclSelectInputDaisy) -> u8 {
        Lpi2c4SclSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4SdaSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_11 for Mode: ALT2"]
    GPIO_EMC_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_13 for Mode: ALT0"]
    GPIO_AD_B0_13_ALT0 = 0x01,
}
impl Lpi2c4SdaSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4SdaSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4SdaSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4SdaSelectInputDaisy {
        Lpi2c4SdaSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpi2c4SdaSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4SdaSelectInputDaisy) -> u8 {
        Lpi2c4SdaSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1Pcs0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT4"]
    GPIO_SD_B0_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_30 for Mode: ALT3"]
    GPIO_EMC_30_ALT3 = 0x01,
}
impl Lpspi1Pcs0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1Pcs0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1Pcs0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1Pcs0SelectInputDaisy {
        Lpspi1Pcs0SelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi1Pcs0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1Pcs0SelectInputDaisy) -> u8 {
        Lpspi1Pcs0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_27 for Mode: ALT3"]
    GPIO_EMC_27_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT4"]
    GPIO_SD_B0_00_ALT4 = 0x01,
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
    #[doc = "Selecting Pad: GPIO_EMC_29 for Mode: ALT3"]
    GPIO_EMC_29_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT4"]
    GPIO_SD_B0_03_ALT4 = 0x01,
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
    #[doc = "Selecting Pad: GPIO_EMC_28 for Mode: ALT3"]
    GPIO_EMC_28_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_02 for Mode: ALT4"]
    GPIO_SD_B0_02_ALT4 = 0x01,
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
pub enum Lpspi2Pcs0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT4"]
    GPIO_SD_B1_06_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_01 for Mode: ALT2"]
    GPIO_EMC_01_ALT2 = 0x01,
}
impl Lpspi2Pcs0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2Pcs0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2Pcs0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2Pcs0SelectInputDaisy {
        Lpspi2Pcs0SelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi2Pcs0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2Pcs0SelectInputDaisy) -> u8 {
        Lpspi2Pcs0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_07 for Mode: ALT4"]
    GPIO_SD_B1_07_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_00 for Mode: ALT2"]
    GPIO_EMC_00_ALT2 = 0x01,
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
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT4"]
    GPIO_SD_B1_09_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_03 for Mode: ALT2"]
    GPIO_EMC_03_ALT2 = 0x01,
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
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT4"]
    GPIO_SD_B1_08_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_02 for Mode: ALT2"]
    GPIO_EMC_02_ALT2 = 0x01,
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
pub enum Lpspi3Pcs0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT7"]
    GPIO_AD_B0_03_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT2"]
    GPIO_AD_B1_12_ALT2 = 0x01,
}
impl Lpspi3Pcs0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3Pcs0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3Pcs0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3Pcs0SelectInputDaisy {
        Lpspi3Pcs0SelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3Pcs0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3Pcs0SelectInputDaisy) -> u8 {
        Lpspi3Pcs0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_00 for Mode: ALT7"]
    GPIO_AD_B0_00_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT2"]
    GPIO_AD_B1_15_ALT2 = 0x01,
}
impl Lpspi3SckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3SckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3SckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3SckSelectInputDaisy {
        Lpspi3SckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3SckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3SckSelectInputDaisy) -> u8 {
        Lpspi3SckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3SdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_02 for Mode: ALT7"]
    GPIO_AD_B0_02_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_13 for Mode: ALT2"]
    GPIO_AD_B1_13_ALT2 = 0x01,
}
impl Lpspi3SdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3SdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3SdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3SdiSelectInputDaisy {
        Lpspi3SdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3SdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3SdiSelectInputDaisy) -> u8 {
        Lpspi3SdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3SdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_01 for Mode: ALT7"]
    GPIO_AD_B0_01_ALT7 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT2"]
    GPIO_AD_B1_14_ALT2 = 0x01,
}
impl Lpspi3SdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3SdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3SdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3SdoSelectInputDaisy {
        Lpspi3SdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi3SdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3SdoSelectInputDaisy) -> u8 {
        Lpspi3SdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4Pcs0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B0_00 for Mode: ALT3"]
    GPIO_B0_00_ALT3 = 0x0,
    #[doc = "Selecting Pad:GPIO_B1_04 for Mode: ALT1"]
    GPIO_B1_04_ALT1 = 0x01,
}
impl Lpspi4Pcs0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4Pcs0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4Pcs0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4Pcs0SelectInputDaisy {
        Lpspi4Pcs0SelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4Pcs0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4Pcs0SelectInputDaisy) -> u8 {
        Lpspi4Pcs0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4SckSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B0_03 for Mode: ALT3"]
    GPIO_B0_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_07 for Mode: ALT1"]
    GPIO_B1_07_ALT1 = 0x01,
}
impl Lpspi4SckSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4SckSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4SckSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4SckSelectInputDaisy {
        Lpspi4SckSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4SckSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4SckSelectInputDaisy) -> u8 {
        Lpspi4SckSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4SdiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B0_01 for Mode: ALT3"]
    GPIO_B0_01_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_05 for Mode: ALT1"]
    GPIO_B1_05_ALT1 = 0x01,
}
impl Lpspi4SdiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4SdiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4SdiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4SdiSelectInputDaisy {
        Lpspi4SdiSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4SdiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4SdiSelectInputDaisy) -> u8 {
        Lpspi4SdiSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4SdoSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_B0_02 for Mode: ALT3"]
    GPIO_B0_02_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_06 for Mode: ALT1"]
    GPIO_B1_06_ALT1 = 0x01,
}
impl Lpspi4SdoSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4SdoSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4SdoSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4SdoSelectInputDaisy {
        Lpspi4SdoSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpspi4SdoSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4SdoSelectInputDaisy) -> u8 {
        Lpspi4SdoSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT2"]
    GPIO_SD_B1_10_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT2"]
    GPIO_AD_B1_03_ALT2 = 0x01,
}
impl Lpuart2RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2RxSelectInputDaisy {
        Lpuart2RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2RxSelectInputDaisy) -> u8 {
        Lpuart2RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_11 for Mode: ALT2"]
    GPIO_SD_B1_11_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT2"]
    GPIO_AD_B1_02_ALT2 = 0x01,
}
impl Lpuart2TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2TxSelectInputDaisy {
        Lpuart2TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart2TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2TxSelectInputDaisy) -> u8 {
        Lpuart2TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3CtsBSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT2"]
    GPIO_EMC_15_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_04 for Mode: ALT2"]
    GPIO_AD_B1_04_ALT2 = 0x01,
}
impl Lpuart3CtsBSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3CtsBSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3CtsBSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3CtsBSelectInputDaisy {
        Lpuart3CtsBSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3CtsBSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3CtsBSelectInputDaisy) -> u8 {
        Lpuart3CtsBSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT2"]
    GPIO_AD_B1_07_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_14 for Mode: ALT2"]
    GPIO_EMC_14_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_09 for Mode: ALT3"]
    GPIO_B0_09_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3RxSelectInputDaisy {
        Lpuart3RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3RxSelectInputDaisy) -> u8 {
        Lpuart3RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT2"]
    GPIO_AD_B1_06_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT2"]
    GPIO_EMC_13_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_08 for Mode: ALT3"]
    GPIO_B0_08_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart3TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3TxSelectInputDaisy {
        Lpuart3TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart3TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3TxSelectInputDaisy) -> u8 {
        Lpuart3TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT4"]
    GPIO_SD_B1_01_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT2"]
    GPIO_EMC_20_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT2"]
    GPIO_B1_01_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart4RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4RxSelectInputDaisy {
        Lpuart4RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4RxSelectInputDaisy) -> u8 {
        Lpuart4RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT4"]
    GPIO_SD_B1_00_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT2"]
    GPIO_EMC_19_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT2"]
    GPIO_B1_00_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart4TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4TxSelectInputDaisy {
        Lpuart4TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart4TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4TxSelectInputDaisy) -> u8 {
        Lpuart4TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_24 for Mode: ALT2"]
    GPIO_EMC_24_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT1"]
    GPIO_B1_13_ALT1 = 0x01,
}
impl Lpuart5RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5RxSelectInputDaisy {
        Lpuart5RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5RxSelectInputDaisy) -> u8 {
        Lpuart5RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_23 for Mode: ALT2"]
    GPIO_EMC_23_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT1"]
    GPIO_B1_12_ALT1 = 0x01,
}
impl Lpuart5TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5TxSelectInputDaisy {
        Lpuart5TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart5TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5TxSelectInputDaisy) -> u8 {
        Lpuart5TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_26 for Mode: ALT2"]
    GPIO_EMC_26_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT2"]
    GPIO_AD_B0_03_ALT2 = 0x01,
}
impl Lpuart6RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6RxSelectInputDaisy {
        Lpuart6RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6RxSelectInputDaisy) -> u8 {
        Lpuart6RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_25 for Mode: ALT2"]
    GPIO_EMC_25_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_02 for Mode: ALT2"]
    GPIO_AD_B0_02_ALT2 = 0x01,
}
impl Lpuart6TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6TxSelectInputDaisy {
        Lpuart6TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart6TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6TxSelectInputDaisy) -> u8 {
        Lpuart6TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT2"]
    GPIO_SD_B1_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_32 for Mode: ALT2"]
    GPIO_EMC_32_ALT2 = 0x01,
}
impl Lpuart7RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7RxSelectInputDaisy {
        Lpuart7RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart7RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7RxSelectInputDaisy) -> u8 {
        Lpuart7RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT2"]
    GPIO_SD_B1_08_ALT2 = 0x0,
    #[doc = "Selecting Pad:GPIO_EMC_31 for Mode: ALT2"]
    GPIO_EMC_31_ALT2 = 0x01,
}
impl Lpuart7TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7TxSelectInputDaisy {
        Lpuart7TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart7TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7TxSelectInputDaisy) -> u8 {
        Lpuart7TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8RxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT2"]
    GPIO_SD_B0_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT2"]
    GPIO_AD_B1_11_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT2"]
    GPIO_EMC_39_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8RxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8RxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8RxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8RxSelectInputDaisy {
        Lpuart8RxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8RxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8RxSelectInputDaisy) -> u8 {
        Lpuart8RxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8TxSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT2"]
    GPIO_SD_B0_04_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT2"]
    GPIO_AD_B1_10_ALT2 = 0x01,
    #[doc = "Selecting Pad: GPIO_EMC_38 for Mode: ALT2"]
    GPIO_EMC_38_ALT2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpuart8TxSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8TxSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8TxSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8TxSelectInputDaisy {
        Lpuart8TxSelectInputDaisy::from_bits(val)
    }
}
impl From<Lpuart8TxSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8TxSelectInputDaisy) -> u8 {
        Lpuart8TxSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NmiSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_12 for Mode: ALT7"]
    GPIO_AD_B0_12_ALT7 = 0x0,
    #[doc = "Selecting Pad: WAKEUP for Mode: ALT7"]
    WAKEUP_ALT7 = 0x01,
}
impl NmiSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NmiSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NmiSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> NmiSelectInputDaisy {
        NmiSelectInputDaisy::from_bits(val)
    }
}
impl From<NmiSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: NmiSelectInputDaisy) -> u8 {
        NmiSelectInputDaisy::to_bits(val)
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
pub enum Qtimer2Timer0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_19 for Mode: ALT4"]
    GPIO_EMC_19_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_03 for Mode: ALT1"]
    GPIO_B0_03_ALT1 = 0x01,
}
impl Qtimer2Timer0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Timer0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Timer0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Timer0SelectInputDaisy {
        Qtimer2Timer0SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Timer0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Timer0SelectInputDaisy) -> u8 {
        Qtimer2Timer0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Timer1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_20 for Mode: ALT4"]
    GPIO_EMC_20_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_04 for Mode: ALT1"]
    GPIO_B0_04_ALT1 = 0x01,
}
impl Qtimer2Timer1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Timer1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Timer1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Timer1SelectInputDaisy {
        Qtimer2Timer1SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Timer1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Timer1SelectInputDaisy) -> u8 {
        Qtimer2Timer1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Timer2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_21 for Mode: ALT4"]
    GPIO_EMC_21_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_05 for Mode: ALT1"]
    GPIO_B0_05_ALT1 = 0x01,
}
impl Qtimer2Timer2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Timer2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Timer2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Timer2SelectInputDaisy {
        Qtimer2Timer2SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Timer2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Timer2SelectInputDaisy) -> u8 {
        Qtimer2Timer2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Timer3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_22 for Mode: ALT4"]
    GPIO_EMC_22_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_09 for Mode: ALT1"]
    GPIO_B1_09_ALT1 = 0x01,
}
impl Qtimer2Timer3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Timer3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Timer3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Timer3SelectInputDaisy {
        Qtimer2Timer3SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer2Timer3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Timer3SelectInputDaisy) -> u8 {
        Qtimer2Timer3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Timer0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT4"]
    GPIO_EMC_15_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_00 for Mode: ALT1"]
    GPIO_AD_B1_00_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_06 for Mode: ALT1"]
    GPIO_B0_06_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Timer0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Timer0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Timer0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Timer0SelectInputDaisy {
        Qtimer3Timer0SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Timer0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Timer0SelectInputDaisy) -> u8 {
        Qtimer3Timer0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Timer1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_01 for Mode: ALT1"]
    GPIO_AD_B1_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT4"]
    GPIO_EMC_16_ALT4 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_07 for Mode: ALT1"]
    GPIO_B0_07_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Timer1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Timer1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Timer1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Timer1SelectInputDaisy {
        Qtimer3Timer1SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Timer1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Timer1SelectInputDaisy) -> u8 {
        Qtimer3Timer1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Timer2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_17 for Mode: ALT4"]
    GPIO_EMC_17_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT1"]
    GPIO_AD_B1_02_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_08 for Mode: ALT1"]
    GPIO_B0_08_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Timer2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Timer2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Timer2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Timer2SelectInputDaisy {
        Qtimer3Timer2SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Timer2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Timer2SelectInputDaisy) -> u8 {
        Qtimer3Timer2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Timer3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_18 for Mode: ALT4"]
    GPIO_EMC_18_ALT4 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT1"]
    GPIO_AD_B1_03_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_10 for Mode: ALT1"]
    GPIO_B1_10_ALT1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Qtimer3Timer3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Timer3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Timer3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Timer3SelectInputDaisy {
        Qtimer3Timer3SelectInputDaisy::from_bits(val)
    }
}
impl From<Qtimer3Timer3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Timer3SelectInputDaisy) -> u8 {
        Qtimer3Timer3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT3"]
    GPIO_SD_B1_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT3"]
    GPIO_AD_B1_09_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_13 for Mode: ALT3"]
    GPIO_B0_13_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1Mclk2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk2SelectInputDaisy {
        Sai1Mclk2SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1Mclk2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk2SelectInputDaisy) -> u8 {
        Sai1Mclk2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT3"]
    GPIO_SD_B1_05_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_11 for Mode: ALT3"]
    GPIO_AD_B1_11_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_15 for Mode: ALT3"]
    GPIO_B0_15_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1RxBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxBclkSelectInputDaisy {
        Sai1RxBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxBclkSelectInputDaisy) -> u8 {
        Sai1RxBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxData0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT3"]
    GPIO_SD_B1_06_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT3"]
    GPIO_AD_B1_12_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_00 for Mode: ALT3"]
    GPIO_B1_00_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1RxData0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxData0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxData0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxData0SelectInputDaisy {
        Sai1RxData0SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxData0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxData0SelectInputDaisy) -> u8 {
        Sai1RxData0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxData1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT3"]
    GPIO_SD_B1_00_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_10 for Mode: ALT3"]
    GPIO_B0_10_ALT3 = 0x01,
}
impl Sai1RxData1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxData1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxData1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxData1SelectInputDaisy {
        Sai1RxData1SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxData1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxData1SelectInputDaisy) -> u8 {
        Sai1RxData1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxData2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT3"]
    GPIO_SD_B1_01_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_11 for Mode: ALT3"]
    GPIO_B0_11_ALT3 = 0x01,
}
impl Sai1RxData2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxData2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxData2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxData2SelectInputDaisy {
        Sai1RxData2SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxData2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxData2SelectInputDaisy) -> u8 {
        Sai1RxData2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxData3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT3"]
    GPIO_SD_B1_02_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B0_12 for Mode: ALT3"]
    GPIO_B0_12_ALT3 = 0x01,
}
impl Sai1RxData3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxData3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxData3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxData3SelectInputDaisy {
        Sai1RxData3SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxData3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxData3SelectInputDaisy) -> u8 {
        Sai1RxData3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1RxSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT3"]
    GPIO_SD_B1_04_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT3"]
    GPIO_AD_B1_10_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B0_14 for Mode: ALT3"]
    GPIO_B0_14_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1RxSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1RxSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1RxSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1RxSyncSelectInputDaisy {
        Sai1RxSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1RxSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1RxSyncSelectInputDaisy) -> u8 {
        Sai1RxSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1TxBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT3"]
    GPIO_SD_B1_08_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT3"]
    GPIO_AD_B1_14_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT3"]
    GPIO_B1_02_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1TxBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1TxBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1TxBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1TxBclkSelectInputDaisy {
        Sai1TxBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1TxBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1TxBclkSelectInputDaisy) -> u8 {
        Sai1TxBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1TxSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT3"]
    GPIO_SD_B1_09_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT3"]
    GPIO_AD_B1_15_ALT3 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT3"]
    GPIO_B1_03_ALT3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1TxSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1TxSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1TxSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai1TxSyncSelectInputDaisy {
        Sai1TxSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai1TxSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai1TxSyncSelectInputDaisy) -> u8 {
        Sai1TxSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2Mclk2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_07 for Mode: ALT2"]
    GPIO_EMC_07_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT3"]
    GPIO_AD_B0_10_ALT3 = 0x01,
}
impl Sai2Mclk2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2Mclk2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2Mclk2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2Mclk2SelectInputDaisy {
        Sai2Mclk2SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2Mclk2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2Mclk2SelectInputDaisy) -> u8 {
        Sai2Mclk2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2RxBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_10 for Mode: ALT2"]
    GPIO_EMC_10_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_06 for Mode: ALT3"]
    GPIO_AD_B0_06_ALT3 = 0x01,
}
impl Sai2RxBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2RxBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2RxBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2RxBclkSelectInputDaisy {
        Sai2RxBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2RxBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2RxBclkSelectInputDaisy) -> u8 {
        Sai2RxBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2RxData0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT2"]
    GPIO_EMC_08_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_08 for Mode: ALT3"]
    GPIO_AD_B0_08_ALT3 = 0x01,
}
impl Sai2RxData0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2RxData0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2RxData0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2RxData0SelectInputDaisy {
        Sai2RxData0SelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2RxData0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2RxData0SelectInputDaisy) -> u8 {
        Sai2RxData0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2RxSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_09 for Mode: ALT2"]
    GPIO_EMC_09_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_07 for Mode: ALT3"]
    GPIO_AD_B0_07_ALT3 = 0x01,
}
impl Sai2RxSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2RxSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2RxSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2RxSyncSelectInputDaisy {
        Sai2RxSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2RxSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2RxSyncSelectInputDaisy) -> u8 {
        Sai2RxSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2TxBclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_06 for Mode: ALT2"]
    GPIO_EMC_06_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT3"]
    GPIO_AD_B0_05_ALT3 = 0x01,
}
impl Sai2TxBclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2TxBclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2TxBclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2TxBclkSelectInputDaisy {
        Sai2TxBclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2TxBclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2TxBclkSelectInputDaisy) -> u8 {
        Sai2TxBclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2TxSyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_05 for Mode: ALT2"]
    GPIO_EMC_05_ALT2 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_04 for Mode: ALT3"]
    GPIO_AD_B0_04_ALT3 = 0x01,
}
impl Sai2TxSyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2TxSyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2TxSyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai2TxSyncSelectInputDaisy {
        Sai2TxSyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai2TxSyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai2TxSyncSelectInputDaisy) -> u8 {
        Sai2TxSyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IpgClkSaiMclkSelectInput2Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT3"]
    GPIO_EMC_37_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT8"]
    GPIO_SD_B1_04_ALT8 = 0x01,
}
impl Sai3IpgClkSaiMclkSelectInput2Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IpgClkSaiMclkSelectInput2Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IpgClkSaiMclkSelectInput2Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IpgClkSaiMclkSelectInput2Daisy {
        Sai3IpgClkSaiMclkSelectInput2Daisy::from_bits(val)
    }
}
impl From<Sai3IpgClkSaiMclkSelectInput2Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IpgClkSaiMclkSelectInput2Daisy) -> u8 {
        Sai3IpgClkSaiMclkSelectInput2Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IppIndSaiRxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT3"]
    GPIO_EMC_35_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_06 for Mode: ALT8"]
    GPIO_SD_B1_06_ALT8 = 0x01,
}
impl Sai3IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IppIndSaiRxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IppIndSaiRxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IppIndSaiRxbclkSelectInputDaisy {
        Sai3IppIndSaiRxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai3IppIndSaiRxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IppIndSaiRxbclkSelectInputDaisy) -> u8 {
        Sai3IppIndSaiRxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IppIndSaiRxdataSelectInput0Daisy {
    #[doc = "Selecting Pad: GPIO_EMC_33 for Mode: ALT3"]
    GPIO_EMC_33_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT8"]
    GPIO_SD_B1_00_ALT8 = 0x01,
}
impl Sai3IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IppIndSaiRxdataSelectInput0Daisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IppIndSaiRxdataSelectInput0Daisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IppIndSaiRxdataSelectInput0Daisy {
        Sai3IppIndSaiRxdataSelectInput0Daisy::from_bits(val)
    }
}
impl From<Sai3IppIndSaiRxdataSelectInput0Daisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IppIndSaiRxdataSelectInput0Daisy) -> u8 {
        Sai3IppIndSaiRxdataSelectInput0Daisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IppIndSaiRxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_34 for Mode: ALT3"]
    GPIO_EMC_34_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT8"]
    GPIO_SD_B1_05_ALT8 = 0x01,
}
impl Sai3IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IppIndSaiRxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IppIndSaiRxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IppIndSaiRxsyncSelectInputDaisy {
        Sai3IppIndSaiRxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai3IppIndSaiRxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IppIndSaiRxsyncSelectInputDaisy) -> u8 {
        Sai3IppIndSaiRxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IppIndSaiTxbclkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_38 for Mode: ALT3"]
    GPIO_EMC_38_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT8"]
    GPIO_SD_B1_03_ALT8 = 0x01,
}
impl Sai3IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IppIndSaiTxbclkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IppIndSaiTxbclkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IppIndSaiTxbclkSelectInputDaisy {
        Sai3IppIndSaiTxbclkSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai3IppIndSaiTxbclkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IppIndSaiTxbclkSelectInputDaisy) -> u8 {
        Sai3IppIndSaiTxbclkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3IppIndSaiTxsyncSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT3"]
    GPIO_EMC_39_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT8"]
    GPIO_SD_B1_02_ALT8 = 0x01,
}
impl Sai3IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3IppIndSaiTxsyncSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3IppIndSaiTxsyncSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Sai3IppIndSaiTxsyncSelectInputDaisy {
        Sai3IppIndSaiTxsyncSelectInputDaisy::from_bits(val)
    }
}
impl From<Sai3IppIndSaiTxsyncSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Sai3IppIndSaiTxsyncSelectInputDaisy) -> u8 {
        Sai3IppIndSaiTxsyncSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcIIppIndDqs4SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT9"]
    GPIO_SD_B0_00_ALT9 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT9"]
    GPIO_EMC_39_ALT9 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT9"]
    GPIO_AD_B0_09_ALT9 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT8"]
    GPIO_B1_13_ALT8 = 0x03,
}
impl SemcIIppIndDqs4SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcIIppIndDqs4SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcIIppIndDqs4SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SemcIIppIndDqs4SelectInputDaisy {
        SemcIIppIndDqs4SelectInputDaisy::from_bits(val)
    }
}
impl From<SemcIIppIndDqs4SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SemcIIppIndDqs4SelectInputDaisy) -> u8 {
        SemcIIppIndDqs4SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpdifInSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT3"]
    GPIO_AD_B1_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT3"]
    GPIO_EMC_16_ALT3 = 0x01,
}
impl SpdifInSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpdifInSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpdifInSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> SpdifInSelectInputDaisy {
        SpdifInSelectInputDaisy::from_bits(val)
    }
}
impl From<SpdifInSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: SpdifInSelectInputDaisy) -> u8 {
        SpdifInSelectInputDaisy::to_bits(val)
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
pub enum UsbOtg1OcSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT3"]
    GPIO_AD_B0_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_03 for Mode: ALT0"]
    GPIO_AD_B1_03_ALT0 = 0x01,
}
impl UsbOtg1OcSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbOtg1OcSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbOtg1OcSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbOtg1OcSelectInputDaisy {
        UsbOtg1OcSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbOtg1OcSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbOtg1OcSelectInputDaisy) -> u8 {
        UsbOtg1OcSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbOtg2OcSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_14 for Mode: ALT0"]
    GPIO_AD_B0_14_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_40 for Mode: ALT3"]
    GPIO_EMC_40_ALT3 = 0x01,
}
impl UsbOtg2OcSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbOtg2OcSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbOtg2OcSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> UsbOtg2OcSelectInputDaisy {
        UsbOtg2OcSelectInputDaisy::from_bits(val)
    }
}
impl From<UsbOtg2OcSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: UsbOtg2OcSelectInputDaisy) -> u8 {
        UsbOtg2OcSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1CdBSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT6"]
    GPIO_EMC_35_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_02 for Mode: ALT6"]
    GPIO_AD_B1_02_ALT6 = 0x01,
    #[doc = "Selecting Pad: GPIO_B1_12 for Mode: ALT6"]
    GPIO_B1_12_ALT6 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usdhc1CdBSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1CdBSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1CdBSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1CdBSelectInputDaisy {
        Usdhc1CdBSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1CdBSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1CdBSelectInputDaisy) -> u8 {
        Usdhc1CdBSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc1WpSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT3"]
    GPIO_EMC_12_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_36for Mode: ALT6"]
    GPIO_EMC_36_ALT6 = 0x01,
    #[doc = "Selecting Pad:GPIO_AD_B1_00 for Mode: ALT6"]
    GPIO_AD_B1_00_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_13 for Mode: ALT6"]
    GPIO_B1_13_ALT6 = 0x03,
}
impl Usdhc1WpSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc1WpSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc1WpSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc1WpSelectInputDaisy {
        Usdhc1WpSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc1WpSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc1WpSelectInputDaisy) -> u8 {
        Usdhc1WpSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2CdBSelectInputDaisy {
    #[doc = "Selecting Pad:GPIO_AD_B1_03 for Mode: ALT6"]
    GPIO_AD_B1_03_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_39 for Mode: ALT6"]
    GPIO_EMC_39_ALT6 = 0x01,
}
impl Usdhc2CdBSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2CdBSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2CdBSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2CdBSelectInputDaisy {
        Usdhc2CdBSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2CdBSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2CdBSelectInputDaisy) -> u8 {
        Usdhc2CdBSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2ClkSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_04 for Mode: ALT0"]
    GPIO_SD_B1_04_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_09 for Mode: ALT6"]
    GPIO_AD_B1_09_ALT6 = 0x01,
}
impl Usdhc2ClkSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2ClkSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2ClkSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2ClkSelectInputDaisy {
        Usdhc2ClkSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2ClkSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2ClkSelectInputDaisy) -> u8 {
        Usdhc2ClkSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2CmdSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_05 for Mode: ALT0"]
    GPIO_SD_B1_05_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_08 for Mode: ALT6"]
    GPIO_AD_B1_08_ALT6 = 0x01,
}
impl Usdhc2CmdSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2CmdSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2CmdSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2CmdSelectInputDaisy {
        Usdhc2CmdSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2CmdSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2CmdSelectInputDaisy) -> u8 {
        Usdhc2CmdSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data0SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_03 for Mode: ALT0"]
    GPIO_SD_B1_03_ALT0 = 0x0,
    #[doc = "Selecting Pad:GPIO_AD_B1_04 for Mode: ALT6"]
    GPIO_AD_B1_04_ALT6 = 0x01,
}
impl Usdhc2Data0SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data0SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data0SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data0SelectInputDaisy {
        Usdhc2Data0SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data0SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data0SelectInputDaisy) -> u8 {
        Usdhc2Data0SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data1SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_02 for Mode: ALT0"]
    GPIO_SD_B1_02_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_05 for Mode: ALT6"]
    GPIO_AD_B1_05_ALT6 = 0x01,
}
impl Usdhc2Data1SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data1SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data1SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data1SelectInputDaisy {
        Usdhc2Data1SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data1SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data1SelectInputDaisy) -> u8 {
        Usdhc2Data1SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data2SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_01 for Mode: ALT0"]
    GPIO_SD_B1_01_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_06 for Mode: ALT6"]
    GPIO_AD_B1_06_ALT6 = 0x01,
}
impl Usdhc2Data2SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data2SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data2SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data2SelectInputDaisy {
        Usdhc2Data2SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data2SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data2SelectInputDaisy) -> u8 {
        Usdhc2Data2SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data3SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_00 for Mode: ALT0"]
    GPIO_SD_B1_00_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_07 for Mode: ALT6"]
    GPIO_AD_B1_07_ALT6 = 0x01,
}
impl Usdhc2Data3SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data3SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data3SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data3SelectInputDaisy {
        Usdhc2Data3SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data3SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data3SelectInputDaisy) -> u8 {
        Usdhc2Data3SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data4SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_08 for Mode: ALT0"]
    GPIO_SD_B1_08_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_12 for Mode: ALT6"]
    GPIO_AD_B1_12_ALT6 = 0x01,
}
impl Usdhc2Data4SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data4SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data4SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data4SelectInputDaisy {
        Usdhc2Data4SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data4SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data4SelectInputDaisy) -> u8 {
        Usdhc2Data4SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data5SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_09 for Mode: ALT0"]
    GPIO_SD_B1_09_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_13 for Mode: ALT6"]
    GPIO_AD_B1_13_ALT6 = 0x01,
}
impl Usdhc2Data5SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data5SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data5SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data5SelectInputDaisy {
        Usdhc2Data5SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data5SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data5SelectInputDaisy) -> u8 {
        Usdhc2Data5SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data6SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_10 for Mode: ALT0"]
    GPIO_SD_B1_10_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_14 for Mode: ALT6"]
    GPIO_AD_B1_14_ALT6 = 0x01,
}
impl Usdhc2Data6SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data6SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data6SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data6SelectInputDaisy {
        Usdhc2Data6SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data6SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data6SelectInputDaisy) -> u8 {
        Usdhc2Data6SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2Data7SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_SD_B1_11 for Mode: ALT0"]
    GPIO_SD_B1_11_ALT0 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_15 for Mode: ALT6"]
    GPIO_AD_B1_15_ALT6 = 0x01,
}
impl Usdhc2Data7SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2Data7SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2Data7SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2Data7SelectInputDaisy {
        Usdhc2Data7SelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2Data7SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2Data7SelectInputDaisy) -> u8 {
        Usdhc2Data7SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usdhc2WpSelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT6"]
    GPIO_EMC_37_ALT6 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B1_10 for Mode: ALT6"]
    GPIO_AD_B1_10_ALT6 = 0x01,
}
impl Usdhc2WpSelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usdhc2WpSelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usdhc2WpSelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Usdhc2WpSelectInputDaisy {
        Usdhc2WpSelectInputDaisy::from_bits(val)
    }
}
impl From<Usdhc2WpSelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Usdhc2WpSelectInputDaisy) -> u8 {
        Usdhc2WpSelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In02SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_00 for Mode: ALT3"]
    GPIO_EMC_00_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_14 for Mode: ALT3"]
    GPIO_B1_14_ALT3 = 0x01,
}
impl Xbar1In02SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In02SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In02SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In02SelectInputDaisy {
        Xbar1In02SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In02SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In02SelectInputDaisy) -> u8 {
        Xbar1In02SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In03SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_01 for Mode: ALT3"]
    GPIO_EMC_01_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_15 for Mode: ALT3"]
    GPIO_B1_15_ALT3 = 0x01,
}
impl Xbar1In03SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In03SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In03SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In03SelectInputDaisy {
        Xbar1In03SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In03SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In03SelectInputDaisy) -> u8 {
        Xbar1In03SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In04SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_02 for Mode: ALT3"]
    GPIO_EMC_02_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_00 for Mode: ALT3"]
    GPIO_SD_B0_00_ALT3 = 0x01,
}
impl Xbar1In04SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In04SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In04SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In04SelectInputDaisy {
        Xbar1In04SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In04SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In04SelectInputDaisy) -> u8 {
        Xbar1In04SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In05SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_03 for Mode: ALT3"]
    GPIO_EMC_03_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_01 for Mode: ALT3"]
    GPIO_SD_B0_01_ALT3 = 0x01,
}
impl Xbar1In05SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In05SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In05SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In05SelectInputDaisy {
        Xbar1In05SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In05SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In05SelectInputDaisy) -> u8 {
        Xbar1In05SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In06SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_04 for Mode: ALT3"]
    GPIO_EMC_04_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_02 for Mode: ALT3"]
    GPIO_SD_B0_02_ALT3 = 0x01,
}
impl Xbar1In06SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In06SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In06SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In06SelectInputDaisy {
        Xbar1In06SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In06SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In06SelectInputDaisy) -> u8 {
        Xbar1In06SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In07SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_05 for Mode: ALT3"]
    GPIO_EMC_05_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_03 for Mode: ALT3"]
    GPIO_SD_B0_03_ALT3 = 0x01,
}
impl Xbar1In07SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In07SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In07SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In07SelectInputDaisy {
        Xbar1In07SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In07SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In07SelectInputDaisy) -> u8 {
        Xbar1In07SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In08SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_06 for Mode: ALT3"]
    GPIO_EMC_06_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_04 for Mode: ALT3"]
    GPIO_SD_B0_04_ALT3 = 0x01,
}
impl Xbar1In08SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In08SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In08SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In08SelectInputDaisy {
        Xbar1In08SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In08SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In08SelectInputDaisy) -> u8 {
        Xbar1In08SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In09SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_07 for Mode: ALT3"]
    GPIO_EMC_07_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_SD_B0_05 for Mode: ALT3"]
    GPIO_SD_B0_05_ALT3 = 0x01,
}
impl Xbar1In09SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In09SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In09SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In09SelectInputDaisy {
        Xbar1In09SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In09SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In09SelectInputDaisy) -> u8 {
        Xbar1In09SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In14SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_00 for Mode: ALT1"]
    GPIO_AD_B0_00_ALT1 = 0x0,
    #[doc = "Selecting Pad:GPIO_B1_00 for Mode: ALT1"]
    GPIO_B1_00_ALT1 = 0x01,
}
impl Xbar1In14SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In14SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In14SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In14SelectInputDaisy {
        Xbar1In14SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In14SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In14SelectInputDaisy) -> u8 {
        Xbar1In14SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In15SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_01 for Mode: ALT1"]
    GPIO_AD_B0_01_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_01 for Mode: ALT1"]
    GPIO_B1_01_ALT1 = 0x01,
}
impl Xbar1In15SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In15SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In15SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In15SelectInputDaisy {
        Xbar1In15SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In15SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In15SelectInputDaisy) -> u8 {
        Xbar1In15SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In16SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_02 for Mode: ALT1"]
    GPIO_AD_B0_02_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_B1_02 for Mode: ALT1"]
    GPIO_B1_02_ALT1 = 0x01,
}
impl Xbar1In16SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In16SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In16SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In16SelectInputDaisy {
        Xbar1In16SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In16SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In16SelectInputDaisy) -> u8 {
        Xbar1In16SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In17SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_08 for Mode: ALT3"]
    GPIO_EMC_08_ALT3 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_03 for Mode: ALT1"]
    GPIO_AD_B0_03_ALT1 = 0x01,
    #[doc = "Selecting Pad: GPIO_AD_B0_05 for Mode: ALT6"]
    GPIO_AD_B0_05_ALT6 = 0x02,
    #[doc = "Selecting Pad: GPIO_B1_03 for Mode: ALT1"]
    GPIO_B1_03_ALT1 = 0x03,
}
impl Xbar1In17SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In17SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In17SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In17SelectInputDaisy {
        Xbar1In17SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In17SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In17SelectInputDaisy) -> u8 {
        Xbar1In17SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In18SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_35 for Mode: ALT1"]
    GPIO_EMC_35_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_06 for Mode: ALT6"]
    GPIO_AD_B0_06_ALT6 = 0x01,
}
impl Xbar1In18SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In18SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In18SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In18SelectInputDaisy {
        Xbar1In18SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In18SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In18SelectInputDaisy) -> u8 {
        Xbar1In18SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In19SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_14 for Mode: ALT1"]
    GPIO_EMC_14_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_07 for Mode: ALT6"]
    GPIO_AD_B0_07_ALT6 = 0x01,
}
impl Xbar1In19SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In19SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In19SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In19SelectInputDaisy {
        Xbar1In19SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In19SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In19SelectInputDaisy) -> u8 {
        Xbar1In19SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In20SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_15 for Mode: ALT1"]
    GPIO_EMC_15_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_08 for Mode: ALT6"]
    GPIO_AD_B0_08_ALT6 = 0x01,
}
impl Xbar1In20SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In20SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In20SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In20SelectInputDaisy {
        Xbar1In20SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In20SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In20SelectInputDaisy) -> u8 {
        Xbar1In20SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In21SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_16 for Mode: ALT1"]
    GPIO_EMC_16_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_09 for Mode: ALT6"]
    GPIO_AD_B0_09_ALT6 = 0x01,
}
impl Xbar1In21SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In21SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In21SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In21SelectInputDaisy {
        Xbar1In21SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In21SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In21SelectInputDaisy) -> u8 {
        Xbar1In21SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In22SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_36 for Mode: ALT1"]
    GPIO_EMC_36_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_10 for Mode: ALT6"]
    GPIO_AD_B0_10_ALT6 = 0x01,
}
impl Xbar1In22SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In22SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In22SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In22SelectInputDaisy {
        Xbar1In22SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In22SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In22SelectInputDaisy) -> u8 {
        Xbar1In22SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In23SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_37 for Mode: ALT1"]
    GPIO_EMC_37_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_11 for Mode: ALT6"]
    GPIO_AD_B0_11_ALT6 = 0x01,
}
impl Xbar1In23SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In23SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In23SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In23SelectInputDaisy {
        Xbar1In23SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In23SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In23SelectInputDaisy) -> u8 {
        Xbar1In23SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In24SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_EMC_12 for Mode: ALT1"]
    GPIO_EMC_12_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_AD_B0_14 for Mode: ALT1"]
    GPIO_AD_B0_14_ALT1 = 0x01,
}
impl Xbar1In24SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In24SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In24SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In24SelectInputDaisy {
        Xbar1In24SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In24SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In24SelectInputDaisy) -> u8 {
        Xbar1In24SelectInputDaisy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xbar1In25SelectInputDaisy {
    #[doc = "Selecting Pad: GPIO_AD_B0_15 for Mode: ALT1"]
    GPIO_AD_B0_15_ALT1 = 0x0,
    #[doc = "Selecting Pad: GPIO_EMC_13 for Mode: ALT1"]
    GPIO_EMC_13_ALT1 = 0x01,
}
impl Xbar1In25SelectInputDaisy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xbar1In25SelectInputDaisy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xbar1In25SelectInputDaisy {
    #[inline(always)]
    fn from(val: u8) -> Xbar1In25SelectInputDaisy {
        Xbar1In25SelectInputDaisy::from_bits(val)
    }
}
impl From<Xbar1In25SelectInputDaisy> for u8 {
    #[inline(always)]
    fn from(val: Xbar1In25SelectInputDaisy) -> u8 {
        Xbar1In25SelectInputDaisy::to_bits(val)
    }
}
