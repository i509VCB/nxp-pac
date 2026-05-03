#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASH_PWRDWN {
    #[doc = "Flash is not in power down mode."]
    PWRUP = 0x0,
    #[doc = "Flash is in power down mode."]
    PWRDWN = 0x01,
}
impl FLASH_PWRDWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASH_PWRDWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASH_PWRDWN {
    #[inline(always)]
    fn from(val: u8) -> FLASH_PWRDWN {
        FLASH_PWRDWN::from_bits(val)
    }
}
impl From<FLASH_PWRDWN> for u8 {
    #[inline(always)]
    fn from(val: FLASH_PWRDWN) -> u8 {
        FLASH_PWRDWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRO192M_TRIM_SRC {
    #[doc = "FRO192M trimming and 'Enable' comes from eFUSE."]
    EFUSE = 0x0,
    #[doc = "FRO192M trimming and 'Enable' comes from FRO192M_CTRL registers."]
    FRO192MCTRL = 0x01,
}
impl FRO192M_TRIM_SRC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRO192M_TRIM_SRC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRO192M_TRIM_SRC {
    #[inline(always)]
    fn from(val: u8) -> FRO192M_TRIM_SRC {
        FRO192M_TRIM_SRC::from_bits(val)
    }
}
impl From<FRO192M_TRIM_SRC> for u8 {
    #[inline(always)]
    fn from(val: FRO192M_TRIM_SRC) -> u8 {
        FRO192M_TRIM_SRC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HIGHZ {
    #[doc = "Output in High normal state."]
    NORMALMPEDANCE = 0x0,
    #[doc = "Output in High Impedance state."]
    HIGHIMPEDANCE = 0x01,
}
impl HIGHZ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HIGHZ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HIGHZ {
    #[inline(always)]
    fn from(val: u8) -> HIGHZ {
        HIGHZ::from_bits(val)
    }
}
impl From<HIGHZ> for u8 {
    #[inline(always)]
    fn from(val: HIGHZ) -> u8 {
        HIGHZ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO0_CTRL_FS {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl RINGO0_CTRL_FS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO0_CTRL_FS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO0_CTRL_FS {
    #[inline(always)]
    fn from(val: u8) -> RINGO0_CTRL_FS {
        RINGO0_CTRL_FS::from_bits(val)
    }
}
impl From<RINGO0_CTRL_FS> for u8 {
    #[inline(always)]
    fn from(val: RINGO0_CTRL_FS) -> u8 {
        RINGO0_CTRL_FS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO0_CTRL_PD {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl RINGO0_CTRL_PD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO0_CTRL_PD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO0_CTRL_PD {
    #[inline(always)]
    fn from(val: u8) -> RINGO0_CTRL_PD {
        RINGO0_CTRL_PD::from_bits(val)
    }
}
impl From<RINGO0_CTRL_PD> for u8 {
    #[inline(always)]
    fn from(val: RINGO0_CTRL_PD) -> u8 {
        RINGO0_CTRL_PD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO1_CTRL_FS {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl RINGO1_CTRL_FS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO1_CTRL_FS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO1_CTRL_FS {
    #[inline(always)]
    fn from(val: u8) -> RINGO1_CTRL_FS {
        RINGO1_CTRL_FS::from_bits(val)
    }
}
impl From<RINGO1_CTRL_FS> for u8 {
    #[inline(always)]
    fn from(val: RINGO1_CTRL_FS) -> u8 {
        RINGO1_CTRL_FS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO1_CTRL_PD {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl RINGO1_CTRL_PD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO1_CTRL_PD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO1_CTRL_PD {
    #[inline(always)]
    fn from(val: u8) -> RINGO1_CTRL_PD {
        RINGO1_CTRL_PD::from_bits(val)
    }
}
impl From<RINGO1_CTRL_PD> for u8 {
    #[inline(always)]
    fn from(val: RINGO1_CTRL_PD) -> u8 {
        RINGO1_CTRL_PD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO1_CTRL_S {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl RINGO1_CTRL_S {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO1_CTRL_S {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO1_CTRL_S {
    #[inline(always)]
    fn from(val: u8) -> RINGO1_CTRL_S {
        RINGO1_CTRL_S::from_bits(val)
    }
}
impl From<RINGO1_CTRL_S> for u8 {
    #[inline(always)]
    fn from(val: RINGO1_CTRL_S) -> u8 {
        RINGO1_CTRL_S::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO2_CTRL_FS {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    FAST = 0x0,
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 0x01,
}
impl RINGO2_CTRL_FS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO2_CTRL_FS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO2_CTRL_FS {
    #[inline(always)]
    fn from(val: u8) -> RINGO2_CTRL_FS {
        RINGO2_CTRL_FS::from_bits(val)
    }
}
impl From<RINGO2_CTRL_FS> for u8 {
    #[inline(always)]
    fn from(val: RINGO2_CTRL_FS) -> u8 {
        RINGO2_CTRL_FS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO2_CTRL_PD {
    #[doc = "The Ringo module is enabled."]
    POWERED_ON = 0x0,
    #[doc = "The Ringo module is disabled."]
    POWERED_DOWN = 0x01,
}
impl RINGO2_CTRL_PD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO2_CTRL_PD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO2_CTRL_PD {
    #[inline(always)]
    fn from(val: u8) -> RINGO2_CTRL_PD {
        RINGO2_CTRL_PD::from_bits(val)
    }
}
impl From<RINGO2_CTRL_PD> for u8 {
    #[inline(always)]
    fn from(val: RINGO2_CTRL_PD) -> u8 {
        RINGO2_CTRL_PD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RINGO2_CTRL_S {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl RINGO2_CTRL_S {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RINGO2_CTRL_S {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RINGO2_CTRL_S {
    #[inline(always)]
    fn from(val: u8) -> RINGO2_CTRL_S {
        RINGO2_CTRL_S::from_bits(val)
    }
}
impl From<RINGO2_CTRL_S> for u8 {
    #[inline(always)]
    fn from(val: RINGO2_CTRL_S) -> u8 {
        RINGO2_CTRL_S::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SL {
    #[doc = "Select short ringo (few elements)."]
    SHORT = 0x0,
    #[doc = "Select long ringo (many elements)."]
    LONG = 0x01,
}
impl SL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SL {
    #[inline(always)]
    fn from(val: u8) -> SL {
        SL::from_bits(val)
    }
}
impl From<SL> for u8 {
    #[inline(always)]
    fn from(val: SL) -> u8 {
        SL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SWN_SWP {
    #[doc = "Normal mode."]
    NORMAL = 0x0,
    #[doc = "P-Monitor mode. Measure with weak P transistor."]
    P_MONITOR = 0x01,
    #[doc = "P-Monitor mode. Measure with weak N transistor."]
    N_MONITOR = 0x02,
    #[doc = "Don't use."]
    FORBIDDEN = 0x03,
}
impl SWN_SWP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SWN_SWP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SWN_SWP {
    #[inline(always)]
    fn from(val: u8) -> SWN_SWP {
        SWN_SWP::from_bits(val)
    }
}
impl From<SWN_SWP> for u8 {
    #[inline(always)]
    fn from(val: SWN_SWP) -> u8 {
        SWN_SWP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VOUT {
    #[doc = "0.750 V."]
    V_0P750 = 0x0,
    #[doc = "0.775 V."]
    V_0P775 = 0x01,
    #[doc = "0.800 V."]
    V_0P800 = 0x02,
    #[doc = "0.825 V."]
    V_0P825 = 0x03,
    #[doc = "0.850 V."]
    V_0P850 = 0x04,
    #[doc = "0.875 V."]
    V_0P875 = 0x05,
    #[doc = "0.900 V."]
    V_0P900 = 0x06,
    #[doc = "0.925 V."]
    V_0P925 = 0x07,
}
impl VOUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VOUT {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VOUT {
    #[inline(always)]
    fn from(val: u8) -> VOUT {
        VOUT::from_bits(val)
    }
}
impl From<VOUT> for u8 {
    #[inline(always)]
    fn from(val: VOUT) -> u8 {
        VOUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XO32M_ADC_CLK_MODE {
    #[doc = "High speed Crystal oscillator output to ADC is disabled."]
    DISABLE = 0x0,
    #[doc = "High speed Crystal oscillator output to ADC is enable."]
    XO_ADC_ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl XO32M_ADC_CLK_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XO32M_ADC_CLK_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XO32M_ADC_CLK_MODE {
    #[inline(always)]
    fn from(val: u8) -> XO32M_ADC_CLK_MODE {
        XO32M_ADC_CLK_MODE::from_bits(val)
    }
}
impl From<XO32M_ADC_CLK_MODE> for u8 {
    #[inline(always)]
    fn from(val: XO32M_ADC_CLK_MODE) -> u8 {
        XO32M_ADC_CLK_MODE::to_bits(val)
    }
}
