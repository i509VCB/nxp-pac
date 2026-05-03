#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADCCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl ADCCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADCCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADCCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> ADCCLKDIV_HALT {
        ADCCLKDIV_HALT::from_bits(val)
    }
}
impl From<ADCCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: ADCCLKDIV_HALT) -> u8 {
        ADCCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADCCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl ADCCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADCCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADCCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> ADCCLKDIV_REQFLAG {
        ADCCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<ADCCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: ADCCLKDIV_REQFLAG) -> u8 {
        ADCCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADCCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl ADCCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADCCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADCCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> ADCCLKDIV_RESET {
        ADCCLKDIV_RESET::from_bits(val)
    }
}
impl From<ADCCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: ADCCLKDIV_RESET) -> u8 {
        ADCCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADCCLKSEL_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Xtal clock coming directly."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl ADCCLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADCCLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADCCLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> ADCCLKSEL_SEL {
        ADCCLKSEL_SEL::from_bits(val)
    }
}
impl From<ADCCLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: ADCCLKSEL_SEL) -> u8 {
        ADCCLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ADC_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl ADC_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ADC_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ADC_RST {
    #[inline(always)]
    fn from(val: u8) -> ADC_RST {
        ADC_RST::from_bits(val)
    }
}
impl From<ADC_RST> for u8 {
    #[inline(always)]
    fn from(val: ADC_RST) -> u8 {
        ADC_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHBCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl AHBCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHBCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHBCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> AHBCLKDIV_HALT {
        AHBCLKDIV_HALT::from_bits(val)
    }
}
impl From<AHBCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: AHBCLKDIV_HALT) -> u8 {
        AHBCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHBCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl AHBCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHBCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHBCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> AHBCLKDIV_REQFLAG {
        AHBCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<AHBCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: AHBCLKDIV_REQFLAG) -> u8 {
        AHBCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHBCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl AHBCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHBCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHBCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> AHBCLKDIV_RESET {
        AHBCLKDIV_RESET::from_bits(val)
    }
}
impl From<AHBCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: AHBCLKDIV_RESET) -> u8 {
        AHBCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANALOG_CTRL_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl ANALOG_CTRL_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANALOG_CTRL_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANALOG_CTRL_RST {
    #[inline(always)]
    fn from(val: u8) -> ANALOG_CTRL_RST {
        ANALOG_CTRL_RST::from_bits(val)
    }
}
impl From<ANALOG_CTRL_RST> for u8 {
    #[inline(always)]
    fn from(val: ANALOG_CTRL_RST) -> u8 {
        ANALOG_CTRL_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AP_FS_DEV_NEEDCLK {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl AP_FS_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AP_FS_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AP_FS_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> AP_FS_DEV_NEEDCLK {
        AP_FS_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<AP_FS_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: AP_FS_DEV_NEEDCLK) -> u8 {
        AP_FS_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AP_FS_HOST_NEEDCLK {
    #[doc = "Under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED = 0x01,
}
impl AP_FS_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AP_FS_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AP_FS_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> AP_FS_HOST_NEEDCLK {
        AP_FS_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<AP_FS_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: AP_FS_HOST_NEEDCLK) -> u8 {
        AP_FS_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AP_HS_DEV_NEEDCLK {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl AP_HS_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AP_HS_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AP_HS_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> AP_HS_DEV_NEEDCLK {
        AP_HS_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<AP_HS_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: AP_HS_DEV_NEEDCLK) -> u8 {
        AP_HS_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AP_HS_HOST_NEEDCLK {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0x0,
    #[doc = "HOST_NEEDCLK is forced high."]
    FORCED = 0x01,
}
impl AP_HS_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AP_HS_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AP_HS_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> AP_HS_HOST_NEEDCLK {
        AP_HS_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<AP_HS_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: AP_HS_HOST_NEEDCLK) -> u8 {
        AP_HS_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CANCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl CANCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CANCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CANCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> CANCLKDIV_HALT {
        CANCLKDIV_HALT::from_bits(val)
    }
}
impl From<CANCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: CANCLKDIV_HALT) -> u8 {
        CANCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CANCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl CANCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CANCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CANCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> CANCLKDIV_REQFLAG {
        CANCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<CANCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: CANCLKDIV_REQFLAG) -> u8 {
        CANCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CANCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl CANCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CANCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CANCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> CANCLKDIV_RESET {
        CANCLKDIV_RESET::from_bits(val)
    }
}
impl From<CANCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: CANCLKDIV_RESET) -> u8 {
        CANCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CANCLKSEL_SEL {
    #[doc = "CAN divided clock."]
    ENUM_0x0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "No clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CANCLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CANCLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CANCLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> CANCLKSEL_SEL {
        CANCLKSEL_SEL::from_bits(val)
    }
}
impl From<CANCLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: CANCLKSEL_SEL) -> u8 {
        CANCLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAN_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CAN_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAN_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAN_RST {
    #[inline(always)]
    fn from(val: u8) -> CAN_RST {
        CAN_RST::from_bits(val)
    }
}
impl From<CAN_RST> for u8 {
    #[inline(always)]
    fn from(val: CAN_RST) -> u8 {
        CAN_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CASPER_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CASPER_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CASPER_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CASPER_RST {
    #[inline(always)]
    fn from(val: u8) -> CASPER_RST {
        CASPER_RST::from_bits(val)
    }
}
impl From<CASPER_RST> for u8 {
    #[inline(always)]
    fn from(val: CASPER_RST) -> u8 {
        CASPER_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CDOG_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CDOG_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CDOG_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CDOG_RST {
    #[inline(always)]
    fn from(val: u8) -> CDOG_RST {
        CDOG_RST::from_bits(val)
    }
}
impl From<CDOG_RST> for u8 {
    #[inline(always)]
    fn from(val: CDOG_RST) -> u8 {
        CDOG_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLK32KCLKSEL_SEL {
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x0 = 0x0,
    #[doc = "FRO1MHz_divided clock."]
    ENUM_0x1 = 0x01,
}
impl CLK32KCLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLK32KCLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLK32KCLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> CLK32KCLKSEL_SEL {
        CLK32KCLKSEL_SEL::from_bits(val)
    }
}
impl From<CLK32KCLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: CLK32KCLKSEL_SEL) -> u8 {
        CLK32KCLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKOUTDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl CLKOUTDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKOUTDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKOUTDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> CLKOUTDIV_HALT {
        CLKOUTDIV_HALT::from_bits(val)
    }
}
impl From<CLKOUTDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: CLKOUTDIV_HALT) -> u8 {
        CLKOUTDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKOUTDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl CLKOUTDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKOUTDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKOUTDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> CLKOUTDIV_REQFLAG {
        CLKOUTDIV_REQFLAG::from_bits(val)
    }
}
impl From<CLKOUTDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: CLKOUTDIV_REQFLAG) -> u8 {
        CLKOUTDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKOUTDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl CLKOUTDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKOUTDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKOUTDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> CLKOUTDIV_RESET {
        CLKOUTDIV_RESET::from_bits(val)
    }
}
impl From<CLKOUTDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: CLKOUTDIV_RESET) -> u8 {
        CLKOUTDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKOUTSEL_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "No clock."]
    ENUM_0xC = 0x0c,
    #[doc = "No clock."]
    ENUM_0xD = 0x0d,
    #[doc = "No clock."]
    ENUM_0xE = 0x0e,
    #[doc = "No clock."]
    ENUM_0xF = 0x0f,
}
impl CLKOUTSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKOUTSEL_SEL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKOUTSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> CLKOUTSEL_SEL {
        CLKOUTSEL_SEL::from_bits(val)
    }
}
impl From<CLKOUTSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: CLKOUTSEL_SEL) -> u8 {
        CLKOUTSEL_SEL::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CLOCKGENUPDATELOCKOUT(u32);
impl CLOCKGENUPDATELOCKOUT {
    #[doc = "all hardware clock configruration are freeze."]
    pub const FREEZE: Self = Self(0x0);
    #[doc = "update all clock configuration."]
    pub const ENABLE: Self = Self(0x01);
}
impl CLOCKGENUPDATELOCKOUT {
    pub const fn from_bits(val: u32) -> CLOCKGENUPDATELOCKOUT {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for CLOCKGENUPDATELOCKOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FREEZE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLOCKGENUPDATELOCKOUT {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FREEZE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for CLOCKGENUPDATELOCKOUT {
    #[inline(always)]
    fn from(val: u32) -> CLOCKGENUPDATELOCKOUT {
        CLOCKGENUPDATELOCKOUT::from_bits(val)
    }
}
impl From<CLOCKGENUPDATELOCKOUT> for u32 {
    #[inline(always)]
    fn from(val: CLOCKGENUPDATELOCKOUT) -> u32 {
        CLOCKGENUPDATELOCKOUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum COMP_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl COMP_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> COMP_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for COMP_RST {
    #[inline(always)]
    fn from(val: u8) -> COMP_RST {
        COMP_RST::from_bits(val)
    }
}
impl From<COMP_RST> for u8 {
    #[inline(always)]
    fn from(val: COMP_RST) -> u8 {
        COMP_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU0LOCKUP {
    #[doc = "the CPU is not in lockup."]
    AWAKE = 0x0,
    #[doc = "the CPU is in lockup."]
    SLEEPING = 0x01,
}
impl CPU0LOCKUP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU0LOCKUP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU0LOCKUP {
    #[inline(always)]
    fn from(val: u8) -> CPU0LOCKUP {
        CPU0LOCKUP::from_bits(val)
    }
}
impl From<CPU0LOCKUP> for u8 {
    #[inline(always)]
    fn from(val: CPU0LOCKUP) -> u8 {
        CPU0LOCKUP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPU0SLEEPING {
    #[doc = "the CPU is not sleeping."]
    AWAKE = 0x0,
    #[doc = "the CPU is sleeping."]
    SLEEPING = 0x01,
}
impl CPU0SLEEPING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPU0SLEEPING {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPU0SLEEPING {
    #[inline(always)]
    fn from(val: u8) -> CPU0SLEEPING {
        CPU0SLEEPING::from_bits(val)
    }
}
impl From<CPU0SLEEPING> for u8 {
    #[inline(always)]
    fn from(val: CPU0SLEEPING) -> u8 {
        CPU0SLEEPING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CRCGEN_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl CRCGEN_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CRCGEN_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CRCGEN_RST {
    #[inline(always)]
    fn from(val: u8) -> CRCGEN_RST {
        CRCGEN_RST::from_bits(val)
    }
}
impl From<CRCGEN_RST> for u8 {
    #[inline(always)]
    fn from(val: CRCGEN_RST) -> u8 {
        CRCGEN_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMERCLKSEL0_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CTIMERCLKSEL0_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMERCLKSEL0_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMERCLKSEL0_SEL {
    #[inline(always)]
    fn from(val: u8) -> CTIMERCLKSEL0_SEL {
        CTIMERCLKSEL0_SEL::from_bits(val)
    }
}
impl From<CTIMERCLKSEL0_SEL> for u8 {
    #[inline(always)]
    fn from(val: CTIMERCLKSEL0_SEL) -> u8 {
        CTIMERCLKSEL0_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMERCLKSEL1_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CTIMERCLKSEL1_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMERCLKSEL1_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMERCLKSEL1_SEL {
    #[inline(always)]
    fn from(val: u8) -> CTIMERCLKSEL1_SEL {
        CTIMERCLKSEL1_SEL::from_bits(val)
    }
}
impl From<CTIMERCLKSEL1_SEL> for u8 {
    #[inline(always)]
    fn from(val: CTIMERCLKSEL1_SEL) -> u8 {
        CTIMERCLKSEL1_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMERCLKSEL2_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CTIMERCLKSEL2_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMERCLKSEL2_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMERCLKSEL2_SEL {
    #[inline(always)]
    fn from(val: u8) -> CTIMERCLKSEL2_SEL {
        CTIMERCLKSEL2_SEL::from_bits(val)
    }
}
impl From<CTIMERCLKSEL2_SEL> for u8 {
    #[inline(always)]
    fn from(val: CTIMERCLKSEL2_SEL) -> u8 {
        CTIMERCLKSEL2_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMERCLKSEL3_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CTIMERCLKSEL3_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMERCLKSEL3_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMERCLKSEL3_SEL {
    #[inline(always)]
    fn from(val: u8) -> CTIMERCLKSEL3_SEL {
        CTIMERCLKSEL3_SEL::from_bits(val)
    }
}
impl From<CTIMERCLKSEL3_SEL> for u8 {
    #[inline(always)]
    fn from(val: CTIMERCLKSEL3_SEL) -> u8 {
        CTIMERCLKSEL3_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTIMERCLKSEL4_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl CTIMERCLKSEL4_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTIMERCLKSEL4_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTIMERCLKSEL4_SEL {
    #[inline(always)]
    fn from(val: u8) -> CTIMERCLKSEL4_SEL {
        CTIMERCLKSEL4_SEL::from_bits(val)
    }
}
impl From<CTIMERCLKSEL4_SEL> for u8 {
    #[inline(always)]
    fn from(val: CTIMERCLKSEL4_SEL) -> u8 {
        CTIMERCLKSEL4_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATACFG {
    #[doc = "Data accesses from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all data accesses."]
    ONEBUF = 0x01,
    #[doc = "All buffers can be used for data accesses."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl DATACFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATACFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATACFG {
    #[inline(always)]
    fn from(val: u8) -> DATACFG {
        DATACFG::from_bits(val)
    }
}
impl From<DATACFG> for u8 {
    #[inline(always)]
    fn from(val: DATACFG) -> u8 {
        DATACFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_CPU0_DBGEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_CPU0_DBGEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_CPU0_DBGEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_CPU0_DBGEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_CPU0_DBGEN {
        DEBUG_FEATURES_CPU0_DBGEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_CPU0_DBGEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_CPU0_DBGEN) -> u8 {
        DEBUG_FEATURES_CPU0_DBGEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_CPU0_NIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_CPU0_NIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_CPU0_NIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_CPU0_NIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_CPU0_NIDEN {
        DEBUG_FEATURES_CPU0_NIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_CPU0_NIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_CPU0_NIDEN) -> u8 {
        DEBUG_FEATURES_CPU0_NIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_CPU0_SPIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_CPU0_SPIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_CPU0_SPIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_CPU0_SPIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_CPU0_SPIDEN {
        DEBUG_FEATURES_CPU0_SPIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_CPU0_SPIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_CPU0_SPIDEN) -> u8 {
        DEBUG_FEATURES_CPU0_SPIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_CPU0_SPNIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_CPU0_SPNIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_CPU0_SPNIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_CPU0_SPNIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_CPU0_SPNIDEN {
        DEBUG_FEATURES_CPU0_SPNIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_CPU0_SPNIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_CPU0_SPNIDEN) -> u8 {
        DEBUG_FEATURES_CPU0_SPNIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_DP_CPU0_DBGEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_DP_CPU0_DBGEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_DP_CPU0_DBGEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_DP_CPU0_DBGEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_DP_CPU0_DBGEN {
        DEBUG_FEATURES_DP_CPU0_DBGEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_DP_CPU0_DBGEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_DP_CPU0_DBGEN) -> u8 {
        DEBUG_FEATURES_DP_CPU0_DBGEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_DP_CPU0_NIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_DP_CPU0_NIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_DP_CPU0_NIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_DP_CPU0_NIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_DP_CPU0_NIDEN {
        DEBUG_FEATURES_DP_CPU0_NIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_DP_CPU0_NIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_DP_CPU0_NIDEN) -> u8 {
        DEBUG_FEATURES_DP_CPU0_NIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_DP_CPU0_SPIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_DP_CPU0_SPIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_DP_CPU0_SPIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_DP_CPU0_SPIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_DP_CPU0_SPIDEN {
        DEBUG_FEATURES_DP_CPU0_SPIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_DP_CPU0_SPIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_DP_CPU0_SPIDEN) -> u8 {
        DEBUG_FEATURES_DP_CPU0_SPIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEBUG_FEATURES_DP_CPU0_SPNIDEN {
    _RESERVED_0 = 0x0,
    #[doc = "Any other value than b10: invasive debug is disable."]
    DISABLE = 0x01,
    #[doc = "10: Invasive debug is enabled."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl DEBUG_FEATURES_DP_CPU0_SPNIDEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEBUG_FEATURES_DP_CPU0_SPNIDEN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEBUG_FEATURES_DP_CPU0_SPNIDEN {
    #[inline(always)]
    fn from(val: u8) -> DEBUG_FEATURES_DP_CPU0_SPNIDEN {
        DEBUG_FEATURES_DP_CPU0_SPNIDEN::from_bits(val)
    }
}
impl From<DEBUG_FEATURES_DP_CPU0_SPNIDEN> for u8 {
    #[inline(always)]
    fn from(val: DEBUG_FEATURES_DP_CPU0_SPNIDEN) -> u8 {
        DEBUG_FEATURES_DP_CPU0_SPNIDEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA0_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl DMA0_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA0_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA0_RST {
    #[inline(always)]
    fn from(val: u8) -> DMA0_RST {
        DMA0_RST::from_bits(val)
    }
}
impl From<DMA0_RST> for u8 {
    #[inline(always)]
    fn from(val: DMA0_RST) -> u8 {
        DMA0_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA1_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl DMA1_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA1_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA1_RST {
    #[inline(always)]
    fn from(val: u8) -> DMA1_RST {
        DMA1_RST::from_bits(val)
    }
}
impl From<DMA1_RST> for u8 {
    #[inline(always)]
    fn from(val: DMA1_RST) -> u8 {
        DMA1_RST::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ENABLEUPDATE(u16);
impl ENABLEUPDATE {
    #[doc = "Bit Fields 0 - 15 of this register are not updated."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Bit Fields 0 - 15 of this register are updated."]
    pub const ENABLE: Self = Self(0xc0de);
}
impl ENABLEUPDATE {
    pub const fn from_bits(val: u16) -> ENABLEUPDATE {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for ENABLEUPDATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0xc0de => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENABLEUPDATE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0xc0de => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for ENABLEUPDATE {
    #[inline(always)]
    fn from(val: u16) -> ENABLEUPDATE {
        ENABLEUPDATE::from_bits(val)
    }
}
impl From<ENABLEUPDATE> for u16 {
    #[inline(always)]
    fn from(val: ENABLEUPDATE) -> u16 {
        ENABLEUPDATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC0_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC0_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC0_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC0_RST {
    #[inline(always)]
    fn from(val: u8) -> FC0_RST {
        FC0_RST::from_bits(val)
    }
}
impl From<FC0_RST> for u8 {
    #[inline(always)]
    fn from(val: FC0_RST) -> u8 {
        FC0_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC1_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC1_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC1_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC1_RST {
    #[inline(always)]
    fn from(val: u8) -> FC1_RST {
        FC1_RST::from_bits(val)
    }
}
impl From<FC1_RST> for u8 {
    #[inline(always)]
    fn from(val: FC1_RST) -> u8 {
        FC1_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC2_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC2_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC2_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC2_RST {
    #[inline(always)]
    fn from(val: u8) -> FC2_RST {
        FC2_RST::from_bits(val)
    }
}
impl From<FC2_RST> for u8 {
    #[inline(always)]
    fn from(val: FC2_RST) -> u8 {
        FC2_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC3_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC3_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC3_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC3_RST {
    #[inline(always)]
    fn from(val: u8) -> FC3_RST {
        FC3_RST::from_bits(val)
    }
}
impl From<FC3_RST> for u8 {
    #[inline(always)]
    fn from(val: FC3_RST) -> u8 {
        FC3_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC4_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC4_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC4_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC4_RST {
    #[inline(always)]
    fn from(val: u8) -> FC4_RST {
        FC4_RST::from_bits(val)
    }
}
impl From<FC4_RST> for u8 {
    #[inline(always)]
    fn from(val: FC4_RST) -> u8 {
        FC4_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC5_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC5_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC5_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC5_RST {
    #[inline(always)]
    fn from(val: u8) -> FC5_RST {
        FC5_RST::from_bits(val)
    }
}
impl From<FC5_RST> for u8 {
    #[inline(always)]
    fn from(val: FC5_RST) -> u8 {
        FC5_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC6_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC6_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC6_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC6_RST {
    #[inline(always)]
    fn from(val: u8) -> FC6_RST {
        FC6_RST::from_bits(val)
    }
}
impl From<FC6_RST> for u8 {
    #[inline(always)]
    fn from(val: FC6_RST) -> u8 {
        FC6_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC7_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FC7_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC7_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC7_RST {
    #[inline(always)]
    fn from(val: u8) -> FC7_RST {
        FC7_RST::from_bits(val)
    }
}
impl From<FC7_RST> for u8 {
    #[inline(always)]
    fn from(val: FC7_RST) -> u8 {
        FC7_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL0_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL0_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL0_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL0_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL0_SEL {
        FCCLKSEL0_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL0_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL0_SEL) -> u8 {
        FCCLKSEL0_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL1_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL1_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL1_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL1_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL1_SEL {
        FCCLKSEL1_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL1_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL1_SEL) -> u8 {
        FCCLKSEL1_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL2_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL2_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL2_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL2_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL2_SEL {
        FCCLKSEL2_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL2_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL2_SEL) -> u8 {
        FCCLKSEL2_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL3_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL3_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL3_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL3_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL3_SEL {
        FCCLKSEL3_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL3_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL3_SEL) -> u8 {
        FCCLKSEL3_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL4_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL4_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL4_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL4_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL4_SEL {
        FCCLKSEL4_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL4_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL4_SEL) -> u8 {
        FCCLKSEL4_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL5_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL5_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL5_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL5_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL5_SEL {
        FCCLKSEL5_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL5_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL5_SEL) -> u8 {
        FCCLKSEL5_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL6_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL6_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL6_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL6_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL6_SEL {
        FCCLKSEL6_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL6_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL6_SEL) -> u8 {
        FCCLKSEL6_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FCCLKSEL7_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl FCCLKSEL7_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FCCLKSEL7_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FCCLKSEL7_SEL {
    #[inline(always)]
    fn from(val: u8) -> FCCLKSEL7_SEL {
        FCCLKSEL7_SEL::from_bits(val)
    }
}
impl From<FCCLKSEL7_SEL> for u8 {
    #[inline(always)]
    fn from(val: FCCLKSEL7_SEL) -> u8 {
        FCCLKSEL7_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FETCHCFG {
    #[doc = "Instruction fetches from flash are not buffered."]
    NOBUF = 0x0,
    #[doc = "One buffer is used for all instruction fetches."]
    ONEBUF = 0x01,
    #[doc = "All buffers may be used for instruction fetches."]
    ALLBUF = 0x02,
    _RESERVED_3 = 0x03,
}
impl FETCHCFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FETCHCFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FETCHCFG {
    #[inline(always)]
    fn from(val: u8) -> FETCHCFG {
        FETCHCFG::from_bits(val)
    }
}
impl From<FETCHCFG> for u8 {
    #[inline(always)]
    fn from(val: FETCHCFG) -> u8 {
        FETCHCFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASHTIM {
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0 = 0x0,
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1 = 0x01,
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2 = 0x02,
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3 = 0x03,
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4 = 0x04,
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5 = 0x05,
    #[doc = "7 system clocks flash access time (for system clock rates up to 84 MHz)."]
    FLASHTIM6 = 0x06,
    #[doc = "8 system clocks flash access time (for system clock rates up to 104 MHz)."]
    FLASHTIM7 = 0x07,
    #[doc = "9 system clocks flash access time (for system clock rates up to 119 MHz)."]
    FLASHTIM8 = 0x08,
    #[doc = "10 system clocks flash access time (for system clock rates up to 129 MHz)."]
    FLASHTIM9 = 0x09,
    #[doc = "11 system clocks flash access time (for system clock rates up to 144 MHz)."]
    FLASHTIM10 = 0x0a,
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    FLASHTIM11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FLASHTIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASHTIM {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASHTIM {
    #[inline(always)]
    fn from(val: u8) -> FLASHTIM {
        FLASHTIM::from_bits(val)
    }
}
impl From<FLASHTIM> for u8 {
    #[inline(always)]
    fn from(val: FLASHTIM) -> u8 {
        FLASHTIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FLASH_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FLASH_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FLASH_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FLASH_RST {
    #[inline(always)]
    fn from(val: u8) -> FLASH_RST {
        FLASH_RST::from_bits(val)
    }
}
impl From<FLASH_RST> for u8 {
    #[inline(always)]
    fn from(val: FLASH_RST) -> u8 {
        FLASH_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FMC_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FMC_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FMC_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FMC_RST {
    #[inline(always)]
    fn from(val: u8) -> FMC_RST {
        FMC_RST::from_bits(val)
    }
}
impl From<FMC_RST> for u8 {
    #[inline(always)]
    fn from(val: FMC_RST) -> u8 {
        FMC_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FREQME_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl FREQME_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FREQME_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FREQME_RST {
    #[inline(always)]
    fn from(val: u8) -> FREQME_RST {
        FREQME_RST::from_bits(val)
    }
}
impl From<FREQME_RST> for u8 {
    #[inline(always)]
    fn from(val: FREQME_RST) -> u8 {
        FREQME_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRO1MCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl FRO1MCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRO1MCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRO1MCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> FRO1MCLKDIV_HALT {
        FRO1MCLKDIV_HALT::from_bits(val)
    }
}
impl From<FRO1MCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: FRO1MCLKDIV_HALT) -> u8 {
        FRO1MCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRO1MCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FRO1MCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRO1MCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRO1MCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> FRO1MCLKDIV_REQFLAG {
        FRO1MCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<FRO1MCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: FRO1MCLKDIV_REQFLAG) -> u8 {
        FRO1MCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRO1MCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FRO1MCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRO1MCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRO1MCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> FRO1MCLKDIV_RESET {
        FRO1MCLKDIV_RESET::from_bits(val)
    }
}
impl From<FRO1MCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: FRO1MCLKDIV_RESET) -> u8 {
        FRO1MCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FROHFDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl FROHFDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FROHFDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FROHFDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> FROHFDIV_HALT {
        FROHFDIV_HALT::from_bits(val)
    }
}
impl From<FROHFDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: FROHFDIV_HALT) -> u8 {
        FROHFDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FROHFDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl FROHFDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FROHFDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FROHFDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> FROHFDIV_REQFLAG {
        FROHFDIV_REQFLAG::from_bits(val)
    }
}
impl From<FROHFDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: FROHFDIV_REQFLAG) -> u8 {
        FROHFDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FROHFDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl FROHFDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FROHFDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FROHFDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> FROHFDIV_RESET {
        FROHFDIV_RESET::from_bits(val)
    }
}
impl From<FROHFDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: FROHFDIV_RESET) -> u8 {
        FROHFDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GINT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GINT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GINT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GINT_RST {
    #[inline(always)]
    fn from(val: u8) -> GINT_RST {
        GINT_RST::from_bits(val)
    }
}
impl From<GINT_RST> for u8 {
    #[inline(always)]
    fn from(val: GINT_RST) -> u8 {
        GINT_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO0_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GPIO0_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO0_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO0_RST {
    #[inline(always)]
    fn from(val: u8) -> GPIO0_RST {
        GPIO0_RST::from_bits(val)
    }
}
impl From<GPIO0_RST> for u8 {
    #[inline(always)]
    fn from(val: GPIO0_RST) -> u8 {
        GPIO0_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO1_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GPIO1_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO1_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO1_RST {
    #[inline(always)]
    fn from(val: u8) -> GPIO1_RST {
        GPIO1_RST::from_bits(val)
    }
}
impl From<GPIO1_RST> for u8 {
    #[inline(always)]
    fn from(val: GPIO1_RST) -> u8 {
        GPIO1_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO_SEC_INT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GPIO_SEC_INT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO_SEC_INT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO_SEC_INT_RST {
    #[inline(always)]
    fn from(val: u8) -> GPIO_SEC_INT_RST {
        GPIO_SEC_INT_RST::from_bits(val)
    }
}
impl From<GPIO_SEC_INT_RST> for u8 {
    #[inline(always)]
    fn from(val: GPIO_SEC_INT_RST) -> u8 {
        GPIO_SEC_INT_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPIO_SEC_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl GPIO_SEC_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPIO_SEC_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPIO_SEC_RST {
    #[inline(always)]
    fn from(val: u8) -> GPIO_SEC_RST {
        GPIO_SEC_RST::from_bits(val)
    }
}
impl From<GPIO_SEC_RST> for u8 {
    #[inline(always)]
    fn from(val: GPIO_SEC_RST) -> u8 {
        GPIO_SEC_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HASH_AES_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HASH_AES_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HASH_AES_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HASH_AES_RST {
    #[inline(always)]
    fn from(val: u8) -> HASH_AES_RST {
        HASH_AES_RST::from_bits(val)
    }
}
impl From<HASH_AES_RST> for u8 {
    #[inline(always)]
    fn from(val: HASH_AES_RST) -> u8 {
        HASH_AES_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HSLSPICLKSEL_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "system PLL divided clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl HSLSPICLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HSLSPICLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HSLSPICLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> HSLSPICLKSEL_SEL {
        HSLSPICLKSEL_SEL::from_bits(val)
    }
}
impl From<HSLSPICLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: HSLSPICLKSEL_SEL) -> u8 {
        HSLSPICLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HS_DEV_WAKEUP_N {
    #[doc = "Forces USB1_PHY to wake-up."]
    FORCE_WUP = 0x0,
    #[doc = "Normal USB1_PHY behavior."]
    NORMAL_WUP = 0x01,
}
impl HS_DEV_WAKEUP_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HS_DEV_WAKEUP_N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HS_DEV_WAKEUP_N {
    #[inline(always)]
    fn from(val: u8) -> HS_DEV_WAKEUP_N {
        HS_DEV_WAKEUP_N::from_bits(val)
    }
}
impl From<HS_DEV_WAKEUP_N> for u8 {
    #[inline(always)]
    fn from(val: HS_DEV_WAKEUP_N) -> u8 {
        HS_DEV_WAKEUP_N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HS_LSPI_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl HS_LSPI_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HS_LSPI_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HS_LSPI_RST {
    #[inline(always)]
    fn from(val: u8) -> HS_LSPI_RST {
        HS_LSPI_RST::from_bits(val)
    }
}
impl From<HS_LSPI_RST> for u8 {
    #[inline(always)]
    fn from(val: HS_LSPI_RST) -> u8 {
        HS_LSPI_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INTERLEAVE {
    #[doc = "RAM access to RAMX0 and RAMX1 is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access to RAMX0 and RAMX1 is interleaved."]
    INTERLEAVE = 0x01,
}
impl INTERLEAVE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INTERLEAVE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INTERLEAVE {
    #[inline(always)]
    fn from(val: u8) -> INTERLEAVE {
        INTERLEAVE::from_bits(val)
    }
}
impl From<INTERLEAVE> for u8 {
    #[inline(always)]
    fn from(val: INTERLEAVE) -> u8 {
        INTERLEAVE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_CLEAR {
    #[doc = "No effect."]
    NONE = 0x0,
    #[doc = "Clear the interrupt. Self-cleared bit."]
    CLEAR = 0x01,
}
impl INT_CLEAR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_CLEAR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_CLEAR {
    #[inline(always)]
    fn from(val: u8) -> INT_CLEAR {
        INT_CLEAR::from_bits(val)
    }
}
impl From<INT_CLEAR> for u8 {
    #[inline(always)]
    fn from(val: INT_CLEAR) -> u8 {
        INT_CLEAR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_CTRL {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE = 0x0,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE = 0x01,
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING = 0x02,
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH = 0x03,
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING = 0x04,
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    LVL_LOW = 0x05,
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH = 0x06,
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2 = 0x07,
}
impl INT_CTRL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_CTRL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_CTRL {
    #[inline(always)]
    fn from(val: u8) -> INT_CTRL {
        INT_CTRL::from_bits(val)
    }
}
impl From<INT_CTRL> for u8 {
    #[inline(always)]
    fn from(val: INT_CTRL) -> u8 {
        INT_CTRL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_ENABLE {
    #[doc = "interrupt disable."]
    INT_DISABLE = 0x0,
    #[doc = "interrupt enable."]
    INT_ENABLE = 0x01,
}
impl INT_ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_ENABLE {
    #[inline(always)]
    fn from(val: u8) -> INT_ENABLE {
        INT_ENABLE::from_bits(val)
    }
}
impl From<INT_ENABLE> for u8 {
    #[inline(always)]
    fn from(val: INT_ENABLE) -> u8 {
        INT_ENABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_SOURCE {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT = 0x0,
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT = 0x01,
}
impl INT_SOURCE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_SOURCE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_SOURCE {
    #[inline(always)]
    fn from(val: u8) -> INT_SOURCE {
        INT_SOURCE::from_bits(val)
    }
}
impl From<INT_SOURCE> for u8 {
    #[inline(always)]
    fn from(val: INT_SOURCE) -> u8 {
        INT_SOURCE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INT_STATUS {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl INT_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INT_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INT_STATUS {
    #[inline(always)]
    fn from(val: u8) -> INT_STATUS {
        INT_STATUS::from_bits(val)
    }
}
impl From<INT_STATUS> for u8 {
    #[inline(always)]
    fn from(val: INT_STATUS) -> u8 {
        INT_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IOCON_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl IOCON_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IOCON_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IOCON_RST {
    #[inline(always)]
    fn from(val: u8) -> IOCON_RST {
        IOCON_RST::from_bits(val)
    }
}
impl From<IOCON_RST> for u8 {
    #[inline(always)]
    fn from(val: IOCON_RST) -> u8 {
        IOCON_RST::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LOCK(u32);
impl LOCK {
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    pub const UNLOCK: Self = Self(0x3cc3_5aa5);
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    pub const LOCK: Self = Self(0xc33c_a55a);
}
impl LOCK {
    pub const fn from_bits(val: u32) -> LOCK {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x3cc3_5aa5 => f.write_str("UNLOCK"),
            0xc33c_a55a => f.write_str("LOCK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOCK {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x3cc3_5aa5 => defmt::write!(f, "UNLOCK"),
            0xc33c_a55a => defmt::write!(f, "LOCK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for LOCK {
    #[inline(always)]
    fn from(val: u32) -> LOCK {
        LOCK::from_bits(val)
    }
}
impl From<LOCK> for u32 {
    #[inline(always)]
    fn from(val: LOCK) -> u32 {
        LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOCK_ALL {
    #[doc = "Any other value than b1010: disable write access to all registers."]
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
    #[doc = "1010: Enable write access to all registers."]
    ENABLE = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LOCK_ALL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOCK_ALL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOCK_ALL {
    #[inline(always)]
    fn from(val: u8) -> LOCK_ALL {
        LOCK_ALL::from_bits(val)
    }
}
impl From<LOCK_ALL> for u8 {
    #[inline(always)]
    fn from(val: LOCK_ALL) -> u8 {
        LOCK_ALL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAILBOX_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MAILBOX_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAILBOX_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAILBOX_RST {
    #[inline(always)]
    fn from(val: u8) -> MAILBOX_RST {
        MAILBOX_RST::from_bits(val)
    }
}
impl From<MAILBOX_RST> for u8 {
    #[inline(always)]
    fn from(val: MAILBOX_RST) -> u8 {
        MAILBOX_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAINCLKSELA_SEL {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MAINCLKSELA_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAINCLKSELA_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAINCLKSELA_SEL {
    #[inline(always)]
    fn from(val: u8) -> MAINCLKSELA_SEL {
        MAINCLKSELA_SEL::from_bits(val)
    }
}
impl From<MAINCLKSELA_SEL> for u8 {
    #[inline(always)]
    fn from(val: MAINCLKSELA_SEL) -> u8 {
        MAINCLKSELA_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAINCLKSELB_SEL {
    #[doc = "Main Clock A."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "PLL1 clock."]
    ENUM_0x2 = 0x02,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MAINCLKSELB_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAINCLKSELB_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAINCLKSELB_SEL {
    #[inline(always)]
    fn from(val: u8) -> MAINCLKSELB_SEL {
        MAINCLKSELB_SEL::from_bits(val)
    }
}
impl From<MAINCLKSELB_SEL> for u8 {
    #[inline(always)]
    fn from(val: MAINCLKSELB_SEL) -> u8 {
        MAINCLKSELB_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MAP {
    #[doc = "Vector Table in ROM."]
    ROM0 = 0x0,
    #[doc = "Vector Table in RAM."]
    RAM1 = 0x01,
    #[doc = "Vector Table in Flash."]
    FLASH0 = 0x02,
    #[doc = "Vector Table in Flash."]
    FLASH1 = 0x03,
}
impl MAP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MAP {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MAP {
    #[inline(always)]
    fn from(val: u8) -> MAP {
        MAP::from_bits(val)
    }
}
impl From<MAP> for u8 {
    #[inline(always)]
    fn from(val: MAP) -> u8 {
        MAP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCLKCLKSEL_SEL {
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl MCLKCLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCLKCLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCLKCLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> MCLKCLKSEL_SEL {
        MCLKCLKSEL_SEL::from_bits(val)
    }
}
impl From<MCLKCLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: MCLKCLKSEL_SEL) -> u8 {
        MCLKCLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl MCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> MCLKDIV_HALT {
        MCLKDIV_HALT::from_bits(val)
    }
}
impl From<MCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: MCLKDIV_HALT) -> u8 {
        MCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl MCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> MCLKDIV_REQFLAG {
        MCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<MCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: MCLKDIV_REQFLAG) -> u8 {
        MCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl MCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> MCLKDIV_RESET {
        MCLKDIV_RESET::from_bits(val)
    }
}
impl From<MCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: MCLKDIV_RESET) -> u8 {
        MCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCLKIO {
    #[doc = "input mode."]
    INPUT = 0x0,
    #[doc = "output mode."]
    OUTPUT = 0x01,
}
impl MCLKIO {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCLKIO {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCLKIO {
    #[inline(always)]
    fn from(val: u8) -> MCLKIO {
        MCLKIO::from_bits(val)
    }
}
impl From<MCLKIO> for u8 {
    #[inline(always)]
    fn from(val: MCLKIO) -> u8 {
        MCLKIO::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MRT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MRT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MRT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MRT_RST {
    #[inline(always)]
    fn from(val: u8) -> MRT_RST {
        MRT_RST::from_bits(val)
    }
}
impl From<MRT_RST> for u8 {
    #[inline(always)]
    fn from(val: MRT_RST) -> u8 {
        MRT_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MUX_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl MUX_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MUX_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MUX_RST {
    #[inline(always)]
    fn from(val: u8) -> MUX_RST {
        MUX_RST::from_bits(val)
    }
}
impl From<MUX_RST> for u8 {
    #[inline(always)]
    fn from(val: MUX_RST) -> u8 {
        MUX_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OSTIMER_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl OSTIMER_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OSTIMER_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OSTIMER_RST {
    #[inline(always)]
    fn from(val: u8) -> OSTIMER_RST {
        OSTIMER_RST::from_bits(val)
    }
}
impl From<OSTIMER_RST> for u8 {
    #[inline(always)]
    fn from(val: OSTIMER_RST) -> u8 {
        OSTIMER_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PINT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PINT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PINT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PINT_RST {
    #[inline(always)]
    fn from(val: u8) -> PINT_RST {
        PINT_RST::from_bits(val)
    }
}
impl From<PINT_RST> for u8 {
    #[inline(always)]
    fn from(val: PINT_RST) -> u8 {
        PINT_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl PLL0CLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> PLL0CLKDIV_HALT {
        PLL0CLKDIV_HALT::from_bits(val)
    }
}
impl From<PLL0CLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: PLL0CLKDIV_HALT) -> u8 {
        PLL0CLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl PLL0CLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> PLL0CLKDIV_REQFLAG {
        PLL0CLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<PLL0CLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: PLL0CLKDIV_REQFLAG) -> u8 {
        PLL0CLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl PLL0CLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> PLL0CLKDIV_RESET {
        PLL0CLKDIV_RESET::from_bits(val)
    }
}
impl From<PLL0CLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: PLL0CLKDIV_RESET) -> u8 {
        PLL0CLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CLKSEL_SEL {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl PLL0CLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL0CLKSEL_SEL {
        PLL0CLKSEL_SEL::from_bits(val)
    }
}
impl From<PLL0CLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL0CLKSEL_SEL) -> u8 {
        PLL0CLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CTRL_BWDIRECT {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl PLL0CTRL_BWDIRECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CTRL_BWDIRECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CTRL_BWDIRECT {
    #[inline(always)]
    fn from(val: u8) -> PLL0CTRL_BWDIRECT {
        PLL0CTRL_BWDIRECT::from_bits(val)
    }
}
impl From<PLL0CTRL_BWDIRECT> for u8 {
    #[inline(always)]
    fn from(val: PLL0CTRL_BWDIRECT) -> u8 {
        PLL0CTRL_BWDIRECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CTRL_BYPASSPLL {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "Bypass PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl PLL0CTRL_BYPASSPLL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CTRL_BYPASSPLL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CTRL_BYPASSPLL {
    #[inline(always)]
    fn from(val: u8) -> PLL0CTRL_BYPASSPLL {
        PLL0CTRL_BYPASSPLL::from_bits(val)
    }
}
impl From<PLL0CTRL_BYPASSPLL> for u8 {
    #[inline(always)]
    fn from(val: PLL0CTRL_BYPASSPLL) -> u8 {
        PLL0CTRL_BYPASSPLL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CTRL_BYPASSPOSTDIV {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl PLL0CTRL_BYPASSPOSTDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CTRL_BYPASSPOSTDIV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CTRL_BYPASSPOSTDIV {
    #[inline(always)]
    fn from(val: u8) -> PLL0CTRL_BYPASSPOSTDIV {
        PLL0CTRL_BYPASSPOSTDIV::from_bits(val)
    }
}
impl From<PLL0CTRL_BYPASSPOSTDIV> for u8 {
    #[inline(always)]
    fn from(val: PLL0CTRL_BYPASSPOSTDIV) -> u8 {
        PLL0CTRL_BYPASSPOSTDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CTRL_BYPASSPOSTDIV2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl PLL0CTRL_BYPASSPOSTDIV2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CTRL_BYPASSPOSTDIV2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CTRL_BYPASSPOSTDIV2 {
    #[inline(always)]
    fn from(val: u8) -> PLL0CTRL_BYPASSPOSTDIV2 {
        PLL0CTRL_BYPASSPOSTDIV2::from_bits(val)
    }
}
impl From<PLL0CTRL_BYPASSPOSTDIV2> for u8 {
    #[inline(always)]
    fn from(val: PLL0CTRL_BYPASSPOSTDIV2) -> u8 {
        PLL0CTRL_BYPASSPOSTDIV2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL0CTRL_BYPASSPREDIV {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl PLL0CTRL_BYPASSPREDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL0CTRL_BYPASSPREDIV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL0CTRL_BYPASSPREDIV {
    #[inline(always)]
    fn from(val: u8) -> PLL0CTRL_BYPASSPREDIV {
        PLL0CTRL_BYPASSPREDIV::from_bits(val)
    }
}
impl From<PLL0CTRL_BYPASSPREDIV> for u8 {
    #[inline(always)]
    fn from(val: PLL0CTRL_BYPASSPREDIV) -> u8 {
        PLL0CTRL_BYPASSPREDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CLKSEL_SEL {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0x0 = 0x0,
    #[doc = "CLKIN clock."]
    ENUM_0x1 = 0x01,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl PLL1CLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL1CLKSEL_SEL {
        PLL1CLKSEL_SEL::from_bits(val)
    }
}
impl From<PLL1CLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL1CLKSEL_SEL) -> u8 {
        PLL1CLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CTRL_BWDIRECT {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0x0,
    #[doc = "modify the bandwidth of the PLL directly."]
    DIRECT = 0x01,
}
impl PLL1CTRL_BWDIRECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CTRL_BWDIRECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CTRL_BWDIRECT {
    #[inline(always)]
    fn from(val: u8) -> PLL1CTRL_BWDIRECT {
        PLL1CTRL_BWDIRECT::from_bits(val)
    }
}
impl From<PLL1CTRL_BWDIRECT> for u8 {
    #[inline(always)]
    fn from(val: PLL1CTRL_BWDIRECT) -> u8 {
        PLL1CTRL_BWDIRECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CTRL_BYPASSPLL {
    #[doc = "use PLL."]
    USED = 0x0,
    #[doc = "PLL input clock is sent directly to the PLL output."]
    BYPASSED = 0x01,
}
impl PLL1CTRL_BYPASSPLL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CTRL_BYPASSPLL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CTRL_BYPASSPLL {
    #[inline(always)]
    fn from(val: u8) -> PLL1CTRL_BYPASSPLL {
        PLL1CTRL_BYPASSPLL::from_bits(val)
    }
}
impl From<PLL1CTRL_BYPASSPLL> for u8 {
    #[inline(always)]
    fn from(val: PLL1CTRL_BYPASSPLL) -> u8 {
        PLL1CTRL_BYPASSPLL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CTRL_BYPASSPOSTDIV {
    #[doc = "use the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the post-divider."]
    BYPASSED = 0x01,
}
impl PLL1CTRL_BYPASSPOSTDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CTRL_BYPASSPOSTDIV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CTRL_BYPASSPOSTDIV {
    #[inline(always)]
    fn from(val: u8) -> PLL1CTRL_BYPASSPOSTDIV {
        PLL1CTRL_BYPASSPOSTDIV::from_bits(val)
    }
}
impl From<PLL1CTRL_BYPASSPOSTDIV> for u8 {
    #[inline(always)]
    fn from(val: PLL1CTRL_BYPASSPOSTDIV) -> u8 {
        PLL1CTRL_BYPASSPOSTDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CTRL_BYPASSPOSTDIV2 {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    USED = 0x0,
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 0x01,
}
impl PLL1CTRL_BYPASSPOSTDIV2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CTRL_BYPASSPOSTDIV2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CTRL_BYPASSPOSTDIV2 {
    #[inline(always)]
    fn from(val: u8) -> PLL1CTRL_BYPASSPOSTDIV2 {
        PLL1CTRL_BYPASSPOSTDIV2::from_bits(val)
    }
}
impl From<PLL1CTRL_BYPASSPOSTDIV2> for u8 {
    #[inline(always)]
    fn from(val: PLL1CTRL_BYPASSPOSTDIV2) -> u8 {
        PLL1CTRL_BYPASSPOSTDIV2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL1CTRL_BYPASSPREDIV {
    #[doc = "use the pre-divider."]
    USED = 0x0,
    #[doc = "bypass of the pre-divider."]
    BYPASSED = 0x01,
}
impl PLL1CTRL_BYPASSPREDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL1CTRL_BYPASSPREDIV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL1CTRL_BYPASSPREDIV {
    #[inline(always)]
    fn from(val: u8) -> PLL1CTRL_BYPASSPREDIV {
        PLL1CTRL_BYPASSPREDIV::from_bits(val)
    }
}
impl From<PLL1CTRL_BYPASSPREDIV> for u8 {
    #[inline(always)]
    fn from(val: PLL1CTRL_BYPASSPREDIV) -> u8 {
        PLL1CTRL_BYPASSPREDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLULUT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PLULUT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLULUT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLULUT_RST {
    #[inline(always)]
    fn from(val: u8) -> PLULUT_RST {
        PLULUT_RST::from_bits(val)
    }
}
impl From<PLULUT_RST> for u8 {
    #[inline(always)]
    fn from(val: PLULUT_RST) -> u8 {
        PLULUT_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum POL_FS_DEV_NEEDCLK {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl POL_FS_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> POL_FS_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for POL_FS_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> POL_FS_DEV_NEEDCLK {
        POL_FS_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<POL_FS_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: POL_FS_DEV_NEEDCLK) -> u8 {
        POL_FS_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum POL_FS_HOST_NEEDCLK {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl POL_FS_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> POL_FS_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for POL_FS_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> POL_FS_HOST_NEEDCLK {
        POL_FS_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<POL_FS_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: POL_FS_HOST_NEEDCLK) -> u8 {
        POL_FS_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum POL_HS_DEV_NEEDCLK {
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl POL_HS_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> POL_HS_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for POL_HS_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> POL_HS_DEV_NEEDCLK {
        POL_HS_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<POL_HS_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: POL_HS_DEV_NEEDCLK) -> u8 {
        POL_HS_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum POL_HS_HOST_NEEDCLK {
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING = 0x0,
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING = 0x01,
}
impl POL_HS_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> POL_HS_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for POL_HS_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> POL_HS_HOST_NEEDCLK {
        POL_HS_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<POL_HS_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: POL_HS_HOST_NEEDCLK) -> u8 {
        POL_HS_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PREFOVR {
    #[doc = "Any previously initiated prefetch will be completed."]
    NORMAL = 0x0,
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    OVERRIDE = 0x01,
}
impl PREFOVR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PREFOVR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PREFOVR {
    #[inline(always)]
    fn from(val: u8) -> PREFOVR {
        PREFOVR::from_bits(val)
    }
}
impl From<PREFOVR> for u8 {
    #[inline(always)]
    fn from(val: PREFOVR) -> u8 {
        PREFOVR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PSYNC {
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    USED = 0x0,
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    BYPASS = 0x01,
}
impl PSYNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PSYNC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PSYNC {
    #[inline(always)]
    fn from(val: u8) -> PSYNC {
        PSYNC::from_bits(val)
    }
}
impl From<PSYNC> for u8 {
    #[inline(always)]
    fn from(val: PSYNC) -> u8 {
        PSYNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PUF_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl PUF_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PUF_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PUF_RST {
    #[inline(always)]
    fn from(val: u8) -> PUF_RST {
        PUF_RST::from_bits(val)
    }
}
impl From<PUF_RST> for u8 {
    #[inline(always)]
    fn from(val: PUF_RST) -> u8 {
        PUF_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RNG_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RNG_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RNG_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RNG_RST {
    #[inline(always)]
    fn from(val: u8) -> RNG_RST {
        RNG_RST::from_bits(val)
    }
}
impl From<RNG_RST> for u8 {
    #[inline(always)]
    fn from(val: RNG_RST) -> u8 {
        RNG_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ROM_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl ROM_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ROM_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ROM_RST {
    #[inline(always)]
    fn from(val: u8) -> ROM_RST {
        ROM_RST::from_bits(val)
    }
}
impl From<ROM_RST> for u8 {
    #[inline(always)]
    fn from(val: ROM_RST) -> u8 {
        ROM_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTC_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl RTC_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTC_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTC_RST {
    #[inline(always)]
    fn from(val: u8) -> RTC_RST {
        RTC_RST::from_bits(val)
    }
}
impl From<RTC_RST> for u8 {
    #[inline(always)]
    fn from(val: RTC_RST) -> u8 {
        RTC_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCTCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl SCTCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCTCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCTCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> SCTCLKDIV_HALT {
        SCTCLKDIV_HALT::from_bits(val)
    }
}
impl From<SCTCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: SCTCLKDIV_HALT) -> u8 {
        SCTCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCTCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SCTCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCTCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCTCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> SCTCLKDIV_REQFLAG {
        SCTCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<SCTCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: SCTCLKDIV_REQFLAG) -> u8 {
        SCTCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCTCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SCTCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCTCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCTCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> SCTCLKDIV_RESET {
        SCTCLKDIV_RESET::from_bits(val)
    }
}
impl From<SCTCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: SCTCLKDIV_RESET) -> u8 {
        SCTCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCTCLKSEL_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "CLKIN clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "MCLK clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl SCTCLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCTCLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCTCLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> SCTCLKSEL_SEL {
        SCTCLKSEL_SEL::from_bits(val)
    }
}
impl From<SCTCLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: SCTCLKSEL_SEL) -> u8 {
        SCTCLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SCT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCT_RST {
    #[inline(always)]
    fn from(val: u8) -> SCT_RST {
        SCT_RST::from_bits(val)
    }
}
impl From<SCT_RST> for u8 {
    #[inline(always)]
    fn from(val: SCT_RST) -> u8 {
        SCT_RST::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SEC_CODE(u32);
impl SEC_CODE {
    #[doc = "CPU0 DAP is not allowed. Reading back register will be read as 0x5."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register will be read as 0xA."]
    pub const ENABLE: Self = Self(0x1234_5678);
}
impl SEC_CODE {
    pub const fn from_bits(val: u32) -> SEC_CODE {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SEC_CODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x1234_5678 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEC_CODE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x1234_5678 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SEC_CODE {
    #[inline(always)]
    fn from(val: u32) -> SEC_CODE {
        SEC_CODE::from_bits(val)
    }
}
impl From<SEC_CODE> for u32 {
    #[inline(always)]
    fn from(val: SEC_CODE) -> u32 {
        SEC_CODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_CTRL1_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SRAM_CTRL1_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_CTRL1_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_CTRL1_RST {
    #[inline(always)]
    fn from(val: u8) -> SRAM_CTRL1_RST {
        SRAM_CTRL1_RST::from_bits(val)
    }
}
impl From<SRAM_CTRL1_RST> for u8 {
    #[inline(always)]
    fn from(val: SRAM_CTRL1_RST) -> u8 {
        SRAM_CTRL1_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRAM_CTRL2_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SRAM_CTRL2_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRAM_CTRL2_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRAM_CTRL2_RST {
    #[inline(always)]
    fn from(val: u8) -> SRAM_CTRL2_RST {
        SRAM_CTRL2_RST::from_bits(val)
    }
}
impl From<SRAM_CTRL2_RST> for u8 {
    #[inline(always)]
    fn from(val: SRAM_CTRL2_RST) -> u8 {
        SRAM_CTRL2_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STATUS {
    #[doc = "no interrupt pending."]
    NO_INT = 0x0,
    #[doc = "interrupt pending."]
    PENDING = 0x01,
}
impl STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STATUS {
    #[inline(always)]
    fn from(val: u8) -> STATUS {
        STATUS::from_bits(val)
    }
}
impl From<STATUS> for u8 {
    #[inline(always)]
    fn from(val: STATUS) -> u8 {
        STATUS::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SWR_RESET(u32);
impl SWR_RESET {
    #[doc = "Bloc is not reset."]
    pub const RELEASED: Self = Self(0x0);
    #[doc = "Generate a software reset."]
    pub const ASSERTED: Self = Self(0x5a00_0001);
}
impl SWR_RESET {
    pub const fn from_bits(val: u32) -> SWR_RESET {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SWR_RESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RELEASED"),
            0x5a00_0001 => f.write_str("ASSERTED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWR_RESET {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RELEASED"),
            0x5a00_0001 => defmt::write!(f, "ASSERTED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SWR_RESET {
    #[inline(always)]
    fn from(val: u32) -> SWR_RESET {
        SWR_RESET::from_bits(val)
    }
}
impl From<SWR_RESET> for u32 {
    #[inline(always)]
    fn from(val: SWR_RESET) -> u32 {
        SWR_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSCTL_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl SYSCTL_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSCTL_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSCTL_RST {
    #[inline(always)]
    fn from(val: u8) -> SYSCTL_RST {
        SYSCTL_RST::from_bits(val)
    }
}
impl From<SYSCTL_RST> for u8 {
    #[inline(always)]
    fn from(val: SYSCTL_RST) -> u8 {
        SYSCTL_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTICKCLKDIV0_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl SYSTICKCLKDIV0_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSTICKCLKDIV0_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSTICKCLKDIV0_HALT {
    #[inline(always)]
    fn from(val: u8) -> SYSTICKCLKDIV0_HALT {
        SYSTICKCLKDIV0_HALT::from_bits(val)
    }
}
impl From<SYSTICKCLKDIV0_HALT> for u8 {
    #[inline(always)]
    fn from(val: SYSTICKCLKDIV0_HALT) -> u8 {
        SYSTICKCLKDIV0_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTICKCLKDIV0_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl SYSTICKCLKDIV0_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSTICKCLKDIV0_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSTICKCLKDIV0_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> SYSTICKCLKDIV0_REQFLAG {
        SYSTICKCLKDIV0_REQFLAG::from_bits(val)
    }
}
impl From<SYSTICKCLKDIV0_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: SYSTICKCLKDIV0_REQFLAG) -> u8 {
        SYSTICKCLKDIV0_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTICKCLKDIV0_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl SYSTICKCLKDIV0_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSTICKCLKDIV0_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSTICKCLKDIV0_RESET {
    #[inline(always)]
    fn from(val: u8) -> SYSTICKCLKDIV0_RESET {
        SYSTICKCLKDIV0_RESET::from_bits(val)
    }
}
impl From<SYSTICKCLKDIV0_RESET> for u8 {
    #[inline(always)]
    fn from(val: SYSTICKCLKDIV0_RESET) -> u8 {
        SYSTICKCLKDIV0_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SYSTICKCLKSEL0_SEL {
    #[doc = "System Tick 0 divided clock."]
    ENUM_0x0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "No clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl SYSTICKCLKSEL0_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SYSTICKCLKSEL0_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SYSTICKCLKSEL0_SEL {
    #[inline(always)]
    fn from(val: u8) -> SYSTICKCLKSEL0_SEL {
        SYSTICKCLKSEL0_SEL::from_bits(val)
    }
}
impl From<SYSTICKCLKSEL0_SEL> for u8 {
    #[inline(always)]
    fn from(val: SYSTICKCLKSEL0_SEL) -> u8 {
        SYSTICKCLKSEL0_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER0_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl TIMER0_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER0_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER0_RST {
    #[inline(always)]
    fn from(val: u8) -> TIMER0_RST {
        TIMER0_RST::from_bits(val)
    }
}
impl From<TIMER0_RST> for u8 {
    #[inline(always)]
    fn from(val: TIMER0_RST) -> u8 {
        TIMER0_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER1_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl TIMER1_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER1_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER1_RST {
    #[inline(always)]
    fn from(val: u8) -> TIMER1_RST {
        TIMER1_RST::from_bits(val)
    }
}
impl From<TIMER1_RST> for u8 {
    #[inline(always)]
    fn from(val: TIMER1_RST) -> u8 {
        TIMER1_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER2_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl TIMER2_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER2_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER2_RST {
    #[inline(always)]
    fn from(val: u8) -> TIMER2_RST {
        TIMER2_RST::from_bits(val)
    }
}
impl From<TIMER2_RST> for u8 {
    #[inline(always)]
    fn from(val: TIMER2_RST) -> u8 {
        TIMER2_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER3_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl TIMER3_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER3_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER3_RST {
    #[inline(always)]
    fn from(val: u8) -> TIMER3_RST {
        TIMER3_RST::from_bits(val)
    }
}
impl From<TIMER3_RST> for u8 {
    #[inline(always)]
    fn from(val: TIMER3_RST) -> u8 {
        TIMER3_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER4_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl TIMER4_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER4_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER4_RST {
    #[inline(always)]
    fn from(val: u8) -> TIMER4_RST {
        TIMER4_RST::from_bits(val)
    }
}
impl From<TIMER4_RST> for u8 {
    #[inline(always)]
    fn from(val: TIMER4_RST) -> u8 {
        TIMER4_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRACECLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl TRACECLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRACECLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRACECLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> TRACECLKDIV_HALT {
        TRACECLKDIV_HALT::from_bits(val)
    }
}
impl From<TRACECLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: TRACECLKDIV_HALT) -> u8 {
        TRACECLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRACECLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl TRACECLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRACECLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRACECLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> TRACECLKDIV_REQFLAG {
        TRACECLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<TRACECLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: TRACECLKDIV_REQFLAG) -> u8 {
        TRACECLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRACECLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl TRACECLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRACECLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRACECLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> TRACECLKDIV_RESET {
        TRACECLKDIV_RESET::from_bits(val)
    }
}
impl From<TRACECLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: TRACECLKDIV_RESET) -> u8 {
        TRACECLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRACECLKSEL_SEL {
    #[doc = "Trace divided clock."]
    ENUM_0x0 = 0x0,
    #[doc = "FRO 1MHz clock."]
    ENUM_0x1 = 0x01,
    #[doc = "Oscillator 32 kHz clock."]
    ENUM_0x2 = 0x02,
    #[doc = "No clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "No clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl TRACECLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRACECLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRACECLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> TRACECLKSEL_SEL {
        TRACECLKSEL_SEL::from_bits(val)
    }
}
impl From<TRACECLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: TRACECLKSEL_SEL) -> u8 {
        TRACECLKSEL_SEL::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UNLOCKCODE(u32);
impl UNLOCKCODE {
    #[doc = "HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    pub const UNLOCK: Self = Self(0xc33c_a55a);
}
impl UNLOCKCODE {
    pub const fn from_bits(val: u32) -> UNLOCKCODE {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for UNLOCKCODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xc33c_a55a => f.write_str("UNLOCK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UNLOCKCODE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xc33c_a55a => defmt::write!(f, "UNLOCK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for UNLOCKCODE {
    #[inline(always)]
    fn from(val: u32) -> UNLOCKCODE {
        UNLOCKCODE::from_bits(val)
    }
}
impl From<UNLOCKCODE> for u32 {
    #[inline(always)]
    fn from(val: UNLOCKCODE) -> u32 {
        UNLOCKCODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0CLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl USB0CLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0CLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0CLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> USB0CLKDIV_HALT {
        USB0CLKDIV_HALT::from_bits(val)
    }
}
impl From<USB0CLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: USB0CLKDIV_HALT) -> u8 {
        USB0CLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0CLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl USB0CLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0CLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0CLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> USB0CLKDIV_REQFLAG {
        USB0CLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<USB0CLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: USB0CLKDIV_REQFLAG) -> u8 {
        USB0CLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0CLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl USB0CLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0CLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0CLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> USB0CLKDIV_RESET {
        USB0CLKDIV_RESET::from_bits(val)
    }
}
impl From<USB0CLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: USB0CLKDIV_RESET) -> u8 {
        USB0CLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0CLKSEL_SEL {
    #[doc = "Main clock."]
    ENUM_0x0 = 0x0,
    #[doc = "PLL0 clock."]
    ENUM_0x1 = 0x01,
    #[doc = "No clock."]
    ENUM_0x2 = 0x02,
    #[doc = "FRO 96 MHz clock."]
    ENUM_0x3 = 0x03,
    #[doc = "No clock."]
    ENUM_0x4 = 0x04,
    #[doc = "PLL1 clock."]
    ENUM_0x5 = 0x05,
    #[doc = "No clock."]
    ENUM_0x6 = 0x06,
    #[doc = "No clock."]
    ENUM_0x7 = 0x07,
}
impl USB0CLKSEL_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0CLKSEL_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0CLKSEL_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB0CLKSEL_SEL {
        USB0CLKSEL_SEL::from_bits(val)
    }
}
impl From<USB0CLKSEL_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB0CLKSEL_SEL) -> u8 {
        USB0CLKSEL_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0NEEDCLKSTAT_DEV_NEEDCLK {
    #[doc = "USB0-FS Device clock is low."]
    LOW = 0x0,
    #[doc = "USB0-FS Device clock is high."]
    HIGH = 0x01,
}
impl USB0NEEDCLKSTAT_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0NEEDCLKSTAT_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0NEEDCLKSTAT_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> USB0NEEDCLKSTAT_DEV_NEEDCLK {
        USB0NEEDCLKSTAT_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<USB0NEEDCLKSTAT_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: USB0NEEDCLKSTAT_DEV_NEEDCLK) -> u8 {
        USB0NEEDCLKSTAT_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0NEEDCLKSTAT_HOST_NEEDCLK {
    #[doc = "USB0-FS Host clock is low."]
    LOW = 0x0,
    #[doc = "USB0-FS Host clock is high."]
    HIGH = 0x01,
}
impl USB0NEEDCLKSTAT_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0NEEDCLKSTAT_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0NEEDCLKSTAT_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> USB0NEEDCLKSTAT_HOST_NEEDCLK {
        USB0NEEDCLKSTAT_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<USB0NEEDCLKSTAT_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: USB0NEEDCLKSTAT_HOST_NEEDCLK) -> u8 {
        USB0NEEDCLKSTAT_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0_DEV_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB0_DEV_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0_DEV_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0_DEV_RST {
    #[inline(always)]
    fn from(val: u8) -> USB0_DEV_RST {
        USB0_DEV_RST::from_bits(val)
    }
}
impl From<USB0_DEV_RST> for u8 {
    #[inline(always)]
    fn from(val: USB0_DEV_RST) -> u8 {
        USB0_DEV_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0_HOSTM_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB0_HOSTM_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0_HOSTM_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0_HOSTM_RST {
    #[inline(always)]
    fn from(val: u8) -> USB0_HOSTM_RST {
        USB0_HOSTM_RST::from_bits(val)
    }
}
impl From<USB0_HOSTM_RST> for u8 {
    #[inline(always)]
    fn from(val: USB0_HOSTM_RST) -> u8 {
        USB0_HOSTM_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB0_HOSTS_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB0_HOSTS_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB0_HOSTS_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB0_HOSTS_RST {
    #[inline(always)]
    fn from(val: u8) -> USB0_HOSTS_RST {
        USB0_HOSTS_RST::from_bits(val)
    }
}
impl From<USB0_HOSTS_RST> for u8 {
    #[inline(always)]
    fn from(val: USB0_HOSTS_RST) -> u8 {
        USB0_HOSTS_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1NEEDCLKSTAT_DEV_NEEDCLK {
    #[doc = "DEV_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "DEV_NEEDCLK is high."]
    HIGH = 0x01,
}
impl USB1NEEDCLKSTAT_DEV_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1NEEDCLKSTAT_DEV_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1NEEDCLKSTAT_DEV_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> USB1NEEDCLKSTAT_DEV_NEEDCLK {
        USB1NEEDCLKSTAT_DEV_NEEDCLK::from_bits(val)
    }
}
impl From<USB1NEEDCLKSTAT_DEV_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: USB1NEEDCLKSTAT_DEV_NEEDCLK) -> u8 {
        USB1NEEDCLKSTAT_DEV_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1NEEDCLKSTAT_HOST_NEEDCLK {
    #[doc = "HOST_NEEDCLK is low."]
    LOW = 0x0,
    #[doc = "HOST_NEEDCLK is high."]
    HIGH = 0x01,
}
impl USB1NEEDCLKSTAT_HOST_NEEDCLK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1NEEDCLKSTAT_HOST_NEEDCLK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1NEEDCLKSTAT_HOST_NEEDCLK {
    #[inline(always)]
    fn from(val: u8) -> USB1NEEDCLKSTAT_HOST_NEEDCLK {
        USB1NEEDCLKSTAT_HOST_NEEDCLK::from_bits(val)
    }
}
impl From<USB1NEEDCLKSTAT_HOST_NEEDCLK> for u8 {
    #[inline(always)]
    fn from(val: USB1NEEDCLKSTAT_HOST_NEEDCLK) -> u8 {
        USB1NEEDCLKSTAT_HOST_NEEDCLK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_DEV_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB1_DEV_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_DEV_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_DEV_RST {
    #[inline(always)]
    fn from(val: u8) -> USB1_DEV_RST {
        USB1_DEV_RST::from_bits(val)
    }
}
impl From<USB1_DEV_RST> for u8 {
    #[inline(always)]
    fn from(val: USB1_DEV_RST) -> u8 {
        USB1_DEV_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_HOST_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB1_HOST_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_HOST_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_HOST_RST {
    #[inline(always)]
    fn from(val: u8) -> USB1_HOST_RST {
        USB1_HOST_RST::from_bits(val)
    }
}
impl From<USB1_HOST_RST> for u8 {
    #[inline(always)]
    fn from(val: USB1_HOST_RST) -> u8 {
        USB1_HOST_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_PHY_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB1_PHY_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_PHY_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_PHY_RST {
    #[inline(always)]
    fn from(val: u8) -> USB1_PHY_RST {
        USB1_PHY_RST::from_bits(val)
    }
}
impl From<USB1_PHY_RST> for u8 {
    #[inline(always)]
    fn from(val: USB1_PHY_RST) -> u8 {
        USB1_PHY_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_RAM_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl USB1_RAM_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_RAM_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_RAM_RST {
    #[inline(always)]
    fn from(val: u8) -> USB1_RAM_RST {
        USB1_RAM_RST::from_bits(val)
    }
}
impl From<USB1_RAM_RST> for u8 {
    #[inline(always)]
    fn from(val: USB1_RAM_RST) -> u8 {
        USB1_RAM_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UTICK_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl UTICK_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UTICK_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UTICK_RST {
    #[inline(always)]
    fn from(val: u8) -> UTICK_RST {
        UTICK_RST::from_bits(val)
    }
}
impl From<UTICK_RST> for u8 {
    #[inline(always)]
    fn from(val: UTICK_RST) -> u8 {
        UTICK_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VAL {
    #[doc = "P+ is smaller than P-."]
    SMALLER = 0x0,
    #[doc = "P+ is greater than P-."]
    GREATER = 0x01,
}
impl VAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VAL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VAL {
    #[inline(always)]
    fn from(val: u8) -> VAL {
        VAL::from_bits(val)
    }
}
impl From<VAL> for u8 {
    #[inline(always)]
    fn from(val: VAL) -> u8 {
        VAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDTCLKDIV_HALT {
    #[doc = "Divider clock is running."]
    RUN = 0x0,
    #[doc = "Divider clock is stoped."]
    HALT = 0x01,
}
impl WDTCLKDIV_HALT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDTCLKDIV_HALT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDTCLKDIV_HALT {
    #[inline(always)]
    fn from(val: u8) -> WDTCLKDIV_HALT {
        WDTCLKDIV_HALT::from_bits(val)
    }
}
impl From<WDTCLKDIV_HALT> for u8 {
    #[inline(always)]
    fn from(val: WDTCLKDIV_HALT) -> u8 {
        WDTCLKDIV_HALT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDTCLKDIV_REQFLAG {
    #[doc = "Divider clock is stable."]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable."]
    ONGOING = 0x01,
}
impl WDTCLKDIV_REQFLAG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDTCLKDIV_REQFLAG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDTCLKDIV_REQFLAG {
    #[inline(always)]
    fn from(val: u8) -> WDTCLKDIV_REQFLAG {
        WDTCLKDIV_REQFLAG::from_bits(val)
    }
}
impl From<WDTCLKDIV_REQFLAG> for u8 {
    #[inline(always)]
    fn from(val: WDTCLKDIV_REQFLAG) -> u8 {
        WDTCLKDIV_REQFLAG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WDTCLKDIV_RESET {
    #[doc = "Divider is not reset."]
    RELEASED = 0x0,
    #[doc = "Divider is reset."]
    ASSERTED = 0x01,
}
impl WDTCLKDIV_RESET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WDTCLKDIV_RESET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WDTCLKDIV_RESET {
    #[inline(always)]
    fn from(val: u8) -> WDTCLKDIV_RESET {
        WDTCLKDIV_RESET::from_bits(val)
    }
}
impl From<WDTCLKDIV_RESET> for u8 {
    #[inline(always)]
    fn from(val: WDTCLKDIV_RESET) -> u8 {
        WDTCLKDIV_RESET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WWDT_RST {
    #[doc = "Bloc is not reset."]
    RELEASED = 0x0,
    #[doc = "Bloc is reset."]
    ASSERTED = 0x01,
}
impl WWDT_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WWDT_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WWDT_RST {
    #[inline(always)]
    fn from(val: u8) -> WWDT_RST {
        WWDT_RST::from_bits(val)
    }
}
impl From<WWDT_RST> for u8 {
    #[inline(always)]
    fn from(val: WWDT_RST) -> u8 {
        WWDT_RST::to_bits(val)
    }
}
