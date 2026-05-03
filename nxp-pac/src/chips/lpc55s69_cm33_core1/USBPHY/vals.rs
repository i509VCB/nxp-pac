#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANACTRL_CLR_DEV_PULLDOWN {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    value0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    value1 = 0x01,
}
impl ANACTRL_CLR_DEV_PULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANACTRL_CLR_DEV_PULLDOWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANACTRL_CLR_DEV_PULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> ANACTRL_CLR_DEV_PULLDOWN {
        ANACTRL_CLR_DEV_PULLDOWN::from_bits(val)
    }
}
impl From<ANACTRL_CLR_DEV_PULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: ANACTRL_CLR_DEV_PULLDOWN) -> u8 {
        ANACTRL_CLR_DEV_PULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANACTRL_DEV_PULLDOWN {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    value0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    value1 = 0x01,
}
impl ANACTRL_DEV_PULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANACTRL_DEV_PULLDOWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANACTRL_DEV_PULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> ANACTRL_DEV_PULLDOWN {
        ANACTRL_DEV_PULLDOWN::from_bits(val)
    }
}
impl From<ANACTRL_DEV_PULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: ANACTRL_DEV_PULLDOWN) -> u8 {
        ANACTRL_DEV_PULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANACTRL_SET_DEV_PULLDOWN {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    value0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    value1 = 0x01,
}
impl ANACTRL_SET_DEV_PULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANACTRL_SET_DEV_PULLDOWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANACTRL_SET_DEV_PULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> ANACTRL_SET_DEV_PULLDOWN {
        ANACTRL_SET_DEV_PULLDOWN::from_bits(val)
    }
}
impl From<ANACTRL_SET_DEV_PULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: ANACTRL_SET_DEV_PULLDOWN) -> u8 {
        ANACTRL_SET_DEV_PULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ANACTRL_TOG_DEV_PULLDOWN {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    value0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    value1 = 0x01,
}
impl ANACTRL_TOG_DEV_PULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ANACTRL_TOG_DEV_PULLDOWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ANACTRL_TOG_DEV_PULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> ANACTRL_TOG_DEV_PULLDOWN {
        ANACTRL_TOG_DEV_PULLDOWN::from_bits(val)
    }
}
impl From<ANACTRL_TOG_DEV_PULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: ANACTRL_TOG_DEV_PULLDOWN) -> u8 {
        ANACTRL_TOG_DEV_PULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTRL_CLR_ENDEVPLUGINDET {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)."]
    value0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins."]
    value1 = 0x01,
}
impl CTRL_CLR_ENDEVPLUGINDET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTRL_CLR_ENDEVPLUGINDET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTRL_CLR_ENDEVPLUGINDET {
    #[inline(always)]
    fn from(val: u8) -> CTRL_CLR_ENDEVPLUGINDET {
        CTRL_CLR_ENDEVPLUGINDET::from_bits(val)
    }
}
impl From<CTRL_CLR_ENDEVPLUGINDET> for u8 {
    #[inline(always)]
    fn from(val: CTRL_CLR_ENDEVPLUGINDET) -> u8 {
        CTRL_CLR_ENDEVPLUGINDET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTRL_ENDEVPLUGINDET {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)."]
    value0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins."]
    value1 = 0x01,
}
impl CTRL_ENDEVPLUGINDET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTRL_ENDEVPLUGINDET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTRL_ENDEVPLUGINDET {
    #[inline(always)]
    fn from(val: u8) -> CTRL_ENDEVPLUGINDET {
        CTRL_ENDEVPLUGINDET::from_bits(val)
    }
}
impl From<CTRL_ENDEVPLUGINDET> for u8 {
    #[inline(always)]
    fn from(val: CTRL_ENDEVPLUGINDET) -> u8 {
        CTRL_ENDEVPLUGINDET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTRL_SET_ENDEVPLUGINDET {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)."]
    value0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins."]
    value1 = 0x01,
}
impl CTRL_SET_ENDEVPLUGINDET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTRL_SET_ENDEVPLUGINDET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTRL_SET_ENDEVPLUGINDET {
    #[inline(always)]
    fn from(val: u8) -> CTRL_SET_ENDEVPLUGINDET {
        CTRL_SET_ENDEVPLUGINDET::from_bits(val)
    }
}
impl From<CTRL_SET_ENDEVPLUGINDET> for u8 {
    #[inline(always)]
    fn from(val: CTRL_SET_ENDEVPLUGINDET) -> u8 {
        CTRL_SET_ENDEVPLUGINDET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CTRL_TOG_ENDEVPLUGINDET {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)."]
    value0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins."]
    value1 = 0x01,
}
impl CTRL_TOG_ENDEVPLUGINDET {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CTRL_TOG_ENDEVPLUGINDET {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CTRL_TOG_ENDEVPLUGINDET {
    #[inline(always)]
    fn from(val: u8) -> CTRL_TOG_ENDEVPLUGINDET {
        CTRL_TOG_ENDEVPLUGINDET::from_bits(val)
    }
}
impl From<CTRL_TOG_ENDEVPLUGINDET> for u8 {
    #[inline(always)]
    fn from(val: CTRL_TOG_ENDEVPLUGINDET) -> u8 {
        CTRL_TOG_ENDEVPLUGINDET::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEVPLUGIN_STATUS {
    #[doc = "No attachment to a USB host is detected."]
    value0 = 0x0,
    #[doc = "Cable attachment to a USB host is detected."]
    value1 = 0x01,
}
impl DEVPLUGIN_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEVPLUGIN_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEVPLUGIN_STATUS {
    #[inline(always)]
    fn from(val: u8) -> DEVPLUGIN_STATUS {
        DEVPLUGIN_STATUS::from_bits(val)
    }
}
impl From<DEVPLUGIN_STATUS> for u8 {
    #[inline(always)]
    fn from(val: DEVPLUGIN_STATUS) -> u8 {
        DEVPLUGIN_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HOSTDISCONDETECT_STATUS {
    #[doc = "USB cable disconnect has not been detected at the local host."]
    value0 = 0x0,
    #[doc = "USB cable disconnect has been detected at the local host."]
    value1 = 0x01,
}
impl HOSTDISCONDETECT_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HOSTDISCONDETECT_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HOSTDISCONDETECT_STATUS {
    #[inline(always)]
    fn from(val: u8) -> HOSTDISCONDETECT_STATUS {
        HOSTDISCONDETECT_STATUS::from_bits(val)
    }
}
impl From<HOSTDISCONDETECT_STATUS> for u8 {
    #[inline(always)]
    fn from(val: HOSTDISCONDETECT_STATUS) -> u8 {
        HOSTDISCONDETECT_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_CLR_PLL_DIV_SEL {
    #[doc = "Divide by 13."]
    value0 = 0x0,
    #[doc = "Divide by 15."]
    value1 = 0x01,
    #[doc = "Divide by 16."]
    value2 = 0x02,
    #[doc = "Divide by 20."]
    value3 = 0x03,
    #[doc = "Divide by 22."]
    value4 = 0x04,
    #[doc = "Divide by 25."]
    value5 = 0x05,
    #[doc = "Divide by 30."]
    value6 = 0x06,
    #[doc = "Divide by 240."]
    value7 = 0x07,
}
impl PLL_SIC_CLR_PLL_DIV_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_CLR_PLL_DIV_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_CLR_PLL_DIV_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_CLR_PLL_DIV_SEL {
        PLL_SIC_CLR_PLL_DIV_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_CLR_PLL_DIV_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_CLR_PLL_DIV_SEL) -> u8 {
        PLL_SIC_CLR_PLL_DIV_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_CLR_PLL_LOCK {
    #[doc = "PLL is not currently locked."]
    value0 = 0x0,
    #[doc = "PLL is currently locked."]
    value1 = 0x01,
}
impl PLL_SIC_CLR_PLL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_CLR_PLL_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_CLR_PLL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_CLR_PLL_LOCK {
        PLL_SIC_CLR_PLL_LOCK::from_bits(val)
    }
}
impl From<PLL_SIC_CLR_PLL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_CLR_PLL_LOCK) -> u8 {
        PLL_SIC_CLR_PLL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_CLR_REFBIAS_PWD_SEL {
    #[doc = "Selects PLL_POWER to control the reference bias."]
    value0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias."]
    value1 = 0x01,
}
impl PLL_SIC_CLR_REFBIAS_PWD_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_CLR_REFBIAS_PWD_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_CLR_REFBIAS_PWD_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_CLR_REFBIAS_PWD_SEL {
        PLL_SIC_CLR_REFBIAS_PWD_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_CLR_REFBIAS_PWD_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_CLR_REFBIAS_PWD_SEL) -> u8 {
        PLL_SIC_CLR_REFBIAS_PWD_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_PLL_DIV_SEL {
    #[doc = "Divide by 13."]
    value0 = 0x0,
    #[doc = "Divide by 15."]
    value1 = 0x01,
    #[doc = "Divide by 16."]
    value2 = 0x02,
    #[doc = "Divide by 20."]
    value3 = 0x03,
    #[doc = "Divide by 22."]
    value4 = 0x04,
    #[doc = "Divide by 25."]
    value5 = 0x05,
    #[doc = "Divide by 30."]
    value6 = 0x06,
    #[doc = "Divide by 240."]
    value7 = 0x07,
}
impl PLL_SIC_PLL_DIV_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_PLL_DIV_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_PLL_DIV_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_PLL_DIV_SEL {
        PLL_SIC_PLL_DIV_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_PLL_DIV_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_PLL_DIV_SEL) -> u8 {
        PLL_SIC_PLL_DIV_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_PLL_LOCK {
    #[doc = "PLL is not currently locked."]
    value0 = 0x0,
    #[doc = "PLL is currently locked."]
    value1 = 0x01,
}
impl PLL_SIC_PLL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_PLL_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_PLL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_PLL_LOCK {
        PLL_SIC_PLL_LOCK::from_bits(val)
    }
}
impl From<PLL_SIC_PLL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_PLL_LOCK) -> u8 {
        PLL_SIC_PLL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_REFBIAS_PWD_SEL {
    #[doc = "Selects PLL_POWER to control the reference bias."]
    value0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias."]
    value1 = 0x01,
}
impl PLL_SIC_REFBIAS_PWD_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_REFBIAS_PWD_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_REFBIAS_PWD_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_REFBIAS_PWD_SEL {
        PLL_SIC_REFBIAS_PWD_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_REFBIAS_PWD_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_REFBIAS_PWD_SEL) -> u8 {
        PLL_SIC_REFBIAS_PWD_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_SET_PLL_DIV_SEL {
    #[doc = "Divide by 13."]
    value0 = 0x0,
    #[doc = "Divide by 15."]
    value1 = 0x01,
    #[doc = "Divide by 16."]
    value2 = 0x02,
    #[doc = "Divide by 20."]
    value3 = 0x03,
    #[doc = "Divide by 22."]
    value4 = 0x04,
    #[doc = "Divide by 25."]
    value5 = 0x05,
    #[doc = "Divide by 30."]
    value6 = 0x06,
    #[doc = "Divide by 240."]
    value7 = 0x07,
}
impl PLL_SIC_SET_PLL_DIV_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_SET_PLL_DIV_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_SET_PLL_DIV_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_SET_PLL_DIV_SEL {
        PLL_SIC_SET_PLL_DIV_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_SET_PLL_DIV_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_SET_PLL_DIV_SEL) -> u8 {
        PLL_SIC_SET_PLL_DIV_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_SET_PLL_LOCK {
    #[doc = "PLL is not currently locked."]
    value0 = 0x0,
    #[doc = "PLL is currently locked."]
    value1 = 0x01,
}
impl PLL_SIC_SET_PLL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_SET_PLL_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_SET_PLL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_SET_PLL_LOCK {
        PLL_SIC_SET_PLL_LOCK::from_bits(val)
    }
}
impl From<PLL_SIC_SET_PLL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_SET_PLL_LOCK) -> u8 {
        PLL_SIC_SET_PLL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_SET_REFBIAS_PWD_SEL {
    #[doc = "Selects PLL_POWER to control the reference bias."]
    value0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias."]
    value1 = 0x01,
}
impl PLL_SIC_SET_REFBIAS_PWD_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_SET_REFBIAS_PWD_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_SET_REFBIAS_PWD_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_SET_REFBIAS_PWD_SEL {
        PLL_SIC_SET_REFBIAS_PWD_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_SET_REFBIAS_PWD_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_SET_REFBIAS_PWD_SEL) -> u8 {
        PLL_SIC_SET_REFBIAS_PWD_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_TOG_PLL_DIV_SEL {
    #[doc = "Divide by 13."]
    value0 = 0x0,
    #[doc = "Divide by 15."]
    value1 = 0x01,
    #[doc = "Divide by 16."]
    value2 = 0x02,
    #[doc = "Divide by 20."]
    value3 = 0x03,
    #[doc = "Divide by 22."]
    value4 = 0x04,
    #[doc = "Divide by 25."]
    value5 = 0x05,
    #[doc = "Divide by 30."]
    value6 = 0x06,
    #[doc = "Divide by 240."]
    value7 = 0x07,
}
impl PLL_SIC_TOG_PLL_DIV_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_TOG_PLL_DIV_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_TOG_PLL_DIV_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_TOG_PLL_DIV_SEL {
        PLL_SIC_TOG_PLL_DIV_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_TOG_PLL_DIV_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_TOG_PLL_DIV_SEL) -> u8 {
        PLL_SIC_TOG_PLL_DIV_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_TOG_PLL_LOCK {
    #[doc = "PLL is not currently locked."]
    value0 = 0x0,
    #[doc = "PLL is currently locked."]
    value1 = 0x01,
}
impl PLL_SIC_TOG_PLL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_TOG_PLL_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_TOG_PLL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_TOG_PLL_LOCK {
        PLL_SIC_TOG_PLL_LOCK::from_bits(val)
    }
}
impl From<PLL_SIC_TOG_PLL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_TOG_PLL_LOCK) -> u8 {
        PLL_SIC_TOG_PLL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_SIC_TOG_REFBIAS_PWD_SEL {
    #[doc = "Selects PLL_POWER to control the reference bias."]
    value0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias."]
    value1 = 0x01,
}
impl PLL_SIC_TOG_REFBIAS_PWD_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_SIC_TOG_REFBIAS_PWD_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_SIC_TOG_REFBIAS_PWD_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_SIC_TOG_REFBIAS_PWD_SEL {
        PLL_SIC_TOG_REFBIAS_PWD_SEL::from_bits(val)
    }
}
impl From<PLL_SIC_TOG_REFBIAS_PWD_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_SIC_TOG_REFBIAS_PWD_SEL) -> u8 {
        PLL_SIC_TOG_REFBIAS_PWD_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_RXPWD1PT1 {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_CLR_RXPWD1PT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_RXPWD1PT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_RXPWD1PT1 {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_RXPWD1PT1 {
        PWD_CLR_RXPWD1PT1::from_bits(val)
    }
}
impl From<PWD_CLR_RXPWD1PT1> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_RXPWD1PT1) -> u8 {
        PWD_CLR_RXPWD1PT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_RXPWDDIFF {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive."]
    value1 = 0x01,
}
impl PWD_CLR_RXPWDDIFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_RXPWDDIFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_RXPWDDIFF {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_RXPWDDIFF {
        PWD_CLR_RXPWDDIFF::from_bits(val)
    }
}
impl From<PWD_CLR_RXPWDDIFF> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_RXPWDDIFF) -> u8 {
        PWD_CLR_RXPWDDIFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_RXPWDENV {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)."]
    value1 = 0x01,
}
impl PWD_CLR_RXPWDENV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_RXPWDENV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_RXPWDENV {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_RXPWDENV {
        PWD_CLR_RXPWDENV::from_bits(val)
    }
}
impl From<PWD_CLR_RXPWDENV> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_RXPWDENV) -> u8 {
        PWD_CLR_RXPWDENV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_RXPWDRX {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_CLR_RXPWDRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_RXPWDRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_RXPWDRX {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_RXPWDRX {
        PWD_CLR_RXPWDRX::from_bits(val)
    }
}
impl From<PWD_CLR_RXPWDRX> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_RXPWDRX) -> u8 {
        PWD_CLR_RXPWDRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_TXPWDFS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the."]
    value1 = 0x01,
}
impl PWD_CLR_TXPWDFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_TXPWDFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_TXPWDFS {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_TXPWDFS {
        PWD_CLR_TXPWDFS::from_bits(val)
    }
}
impl From<PWD_CLR_TXPWDFS> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_TXPWDFS) -> u8 {
        PWD_CLR_TXPWDFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_TXPWDIBIAS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the."]
    value1 = 0x01,
}
impl PWD_CLR_TXPWDIBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_TXPWDIBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_TXPWDIBIAS {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_TXPWDIBIAS {
        PWD_CLR_TXPWDIBIAS::from_bits(val)
    }
}
impl From<PWD_CLR_TXPWDIBIAS> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_TXPWDIBIAS) -> u8 {
        PWD_CLR_TXPWDIBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_CLR_TXPWDV2I {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror."]
    value1 = 0x01,
}
impl PWD_CLR_TXPWDV2I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_CLR_TXPWDV2I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_CLR_TXPWDV2I {
    #[inline(always)]
    fn from(val: u8) -> PWD_CLR_TXPWDV2I {
        PWD_CLR_TXPWDV2I::from_bits(val)
    }
}
impl From<PWD_CLR_TXPWDV2I> for u8 {
    #[inline(always)]
    fn from(val: PWD_CLR_TXPWDV2I) -> u8 {
        PWD_CLR_TXPWDV2I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_RXPWD1PT1 {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_RXPWD1PT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_RXPWD1PT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_RXPWD1PT1 {
    #[inline(always)]
    fn from(val: u8) -> PWD_RXPWD1PT1 {
        PWD_RXPWD1PT1::from_bits(val)
    }
}
impl From<PWD_RXPWD1PT1> for u8 {
    #[inline(always)]
    fn from(val: PWD_RXPWD1PT1) -> u8 {
        PWD_RXPWD1PT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_RXPWDDIFF {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive."]
    value1 = 0x01,
}
impl PWD_RXPWDDIFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_RXPWDDIFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_RXPWDDIFF {
    #[inline(always)]
    fn from(val: u8) -> PWD_RXPWDDIFF {
        PWD_RXPWDDIFF::from_bits(val)
    }
}
impl From<PWD_RXPWDDIFF> for u8 {
    #[inline(always)]
    fn from(val: PWD_RXPWDDIFF) -> u8 {
        PWD_RXPWDDIFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_RXPWDENV {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)."]
    value1 = 0x01,
}
impl PWD_RXPWDENV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_RXPWDENV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_RXPWDENV {
    #[inline(always)]
    fn from(val: u8) -> PWD_RXPWDENV {
        PWD_RXPWDENV::from_bits(val)
    }
}
impl From<PWD_RXPWDENV> for u8 {
    #[inline(always)]
    fn from(val: PWD_RXPWDENV) -> u8 {
        PWD_RXPWDENV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_RXPWDRX {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_RXPWDRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_RXPWDRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_RXPWDRX {
    #[inline(always)]
    fn from(val: u8) -> PWD_RXPWDRX {
        PWD_RXPWDRX::from_bits(val)
    }
}
impl From<PWD_RXPWDRX> for u8 {
    #[inline(always)]
    fn from(val: PWD_RXPWDRX) -> u8 {
        PWD_RXPWDRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_RXPWD1PT1 {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_SET_RXPWD1PT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_RXPWD1PT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_RXPWD1PT1 {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_RXPWD1PT1 {
        PWD_SET_RXPWD1PT1::from_bits(val)
    }
}
impl From<PWD_SET_RXPWD1PT1> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_RXPWD1PT1) -> u8 {
        PWD_SET_RXPWD1PT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_RXPWDDIFF {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive."]
    value1 = 0x01,
}
impl PWD_SET_RXPWDDIFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_RXPWDDIFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_RXPWDDIFF {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_RXPWDDIFF {
        PWD_SET_RXPWDDIFF::from_bits(val)
    }
}
impl From<PWD_SET_RXPWDDIFF> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_RXPWDDIFF) -> u8 {
        PWD_SET_RXPWDDIFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_RXPWDENV {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)."]
    value1 = 0x01,
}
impl PWD_SET_RXPWDENV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_RXPWDENV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_RXPWDENV {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_RXPWDENV {
        PWD_SET_RXPWDENV::from_bits(val)
    }
}
impl From<PWD_SET_RXPWDENV> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_RXPWDENV) -> u8 {
        PWD_SET_RXPWDENV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_RXPWDRX {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_SET_RXPWDRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_RXPWDRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_RXPWDRX {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_RXPWDRX {
        PWD_SET_RXPWDRX::from_bits(val)
    }
}
impl From<PWD_SET_RXPWDRX> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_RXPWDRX) -> u8 {
        PWD_SET_RXPWDRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_TXPWDFS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the."]
    value1 = 0x01,
}
impl PWD_SET_TXPWDFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_TXPWDFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_TXPWDFS {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_TXPWDFS {
        PWD_SET_TXPWDFS::from_bits(val)
    }
}
impl From<PWD_SET_TXPWDFS> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_TXPWDFS) -> u8 {
        PWD_SET_TXPWDFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_TXPWDIBIAS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the."]
    value1 = 0x01,
}
impl PWD_SET_TXPWDIBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_TXPWDIBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_TXPWDIBIAS {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_TXPWDIBIAS {
        PWD_SET_TXPWDIBIAS::from_bits(val)
    }
}
impl From<PWD_SET_TXPWDIBIAS> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_TXPWDIBIAS) -> u8 {
        PWD_SET_TXPWDIBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_SET_TXPWDV2I {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror."]
    value1 = 0x01,
}
impl PWD_SET_TXPWDV2I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_SET_TXPWDV2I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_SET_TXPWDV2I {
    #[inline(always)]
    fn from(val: u8) -> PWD_SET_TXPWDV2I {
        PWD_SET_TXPWDV2I::from_bits(val)
    }
}
impl From<PWD_SET_TXPWDV2I> for u8 {
    #[inline(always)]
    fn from(val: PWD_SET_TXPWDV2I) -> u8 {
        PWD_SET_TXPWDV2I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_RXPWD1PT1 {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_TOG_RXPWD1PT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_RXPWD1PT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_RXPWD1PT1 {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_RXPWD1PT1 {
        PWD_TOG_RXPWD1PT1::from_bits(val)
    }
}
impl From<PWD_TOG_RXPWD1PT1> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_RXPWD1PT1) -> u8 {
        PWD_TOG_RXPWD1PT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_RXPWDDIFF {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive."]
    value1 = 0x01,
}
impl PWD_TOG_RXPWDDIFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_RXPWDDIFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_RXPWDDIFF {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_RXPWDDIFF {
        PWD_TOG_RXPWDDIFF::from_bits(val)
    }
}
impl From<PWD_TOG_RXPWDDIFF> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_RXPWDDIFF) -> u8 {
        PWD_TOG_RXPWDDIFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_RXPWDENV {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)."]
    value1 = 0x01,
}
impl PWD_TOG_RXPWDENV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_RXPWDENV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_RXPWDENV {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_RXPWDENV {
        PWD_TOG_RXPWDENV::from_bits(val)
    }
}
impl From<PWD_TOG_RXPWDENV> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_RXPWDENV) -> u8 {
        PWD_TOG_RXPWDENV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_RXPWDRX {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver."]
    value1 = 0x01,
}
impl PWD_TOG_RXPWDRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_RXPWDRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_RXPWDRX {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_RXPWDRX {
        PWD_TOG_RXPWDRX::from_bits(val)
    }
}
impl From<PWD_TOG_RXPWDRX> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_RXPWDRX) -> u8 {
        PWD_TOG_RXPWDRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_TXPWDFS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the."]
    value1 = 0x01,
}
impl PWD_TOG_TXPWDFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_TXPWDFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_TXPWDFS {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_TXPWDFS {
        PWD_TOG_TXPWDFS::from_bits(val)
    }
}
impl From<PWD_TOG_TXPWDFS> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_TXPWDFS) -> u8 {
        PWD_TOG_TXPWDFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_TXPWDIBIAS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the."]
    value1 = 0x01,
}
impl PWD_TOG_TXPWDIBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_TXPWDIBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_TXPWDIBIAS {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_TXPWDIBIAS {
        PWD_TOG_TXPWDIBIAS::from_bits(val)
    }
}
impl From<PWD_TOG_TXPWDIBIAS> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_TXPWDIBIAS) -> u8 {
        PWD_TOG_TXPWDIBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TOG_TXPWDV2I {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror."]
    value1 = 0x01,
}
impl PWD_TOG_TXPWDV2I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TOG_TXPWDV2I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TOG_TXPWDV2I {
    #[inline(always)]
    fn from(val: u8) -> PWD_TOG_TXPWDV2I {
        PWD_TOG_TXPWDV2I::from_bits(val)
    }
}
impl From<PWD_TOG_TXPWDV2I> for u8 {
    #[inline(always)]
    fn from(val: PWD_TOG_TXPWDV2I) -> u8 {
        PWD_TOG_TXPWDV2I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TXPWDFS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the."]
    value1 = 0x01,
}
impl PWD_TXPWDFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TXPWDFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TXPWDFS {
    #[inline(always)]
    fn from(val: u8) -> PWD_TXPWDFS {
        PWD_TXPWDFS::from_bits(val)
    }
}
impl From<PWD_TXPWDFS> for u8 {
    #[inline(always)]
    fn from(val: PWD_TXPWDFS) -> u8 {
        PWD_TXPWDFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TXPWDIBIAS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the."]
    value1 = 0x01,
}
impl PWD_TXPWDIBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TXPWDIBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TXPWDIBIAS {
    #[inline(always)]
    fn from(val: u8) -> PWD_TXPWDIBIAS {
        PWD_TXPWDIBIAS::from_bits(val)
    }
}
impl From<PWD_TXPWDIBIAS> for u8 {
    #[inline(always)]
    fn from(val: PWD_TXPWDIBIAS) -> u8 {
        PWD_TXPWDIBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PWD_TXPWDV2I {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror."]
    value1 = 0x01,
}
impl PWD_TXPWDV2I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PWD_TXPWDV2I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PWD_TXPWDV2I {
    #[inline(always)]
    fn from(val: u8) -> PWD_TXPWDV2I {
        PWD_TXPWDV2I::from_bits(val)
    }
}
impl From<PWD_TXPWDV2I> for u8 {
    #[inline(always)]
    fn from(val: PWD_TXPWDV2I) -> u8 {
        PWD_TXPWDV2I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_CLR_DISCONADJ {
    #[doc = "Trip-Level Voltage is 0.56875 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_CLR_DISCONADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_CLR_DISCONADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_CLR_DISCONADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_CLR_DISCONADJ {
        RX_CLR_DISCONADJ::from_bits(val)
    }
}
impl From<RX_CLR_DISCONADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_CLR_DISCONADJ) -> u8 {
        RX_CLR_DISCONADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_CLR_ENVADJ {
    #[doc = "Trip-Level Voltage is 0.1000 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_CLR_ENVADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_CLR_ENVADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_CLR_ENVADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_CLR_ENVADJ {
        RX_CLR_ENVADJ::from_bits(val)
    }
}
impl From<RX_CLR_ENVADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_CLR_ENVADJ) -> u8 {
        RX_CLR_ENVADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_CLR_RXDBYPASS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver."]
    value1 = 0x01,
}
impl RX_CLR_RXDBYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_CLR_RXDBYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_CLR_RXDBYPASS {
    #[inline(always)]
    fn from(val: u8) -> RX_CLR_RXDBYPASS {
        RX_CLR_RXDBYPASS::from_bits(val)
    }
}
impl From<RX_CLR_RXDBYPASS> for u8 {
    #[inline(always)]
    fn from(val: RX_CLR_RXDBYPASS) -> u8 {
        RX_CLR_RXDBYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_DISCONADJ {
    #[doc = "Trip-Level Voltage is 0.56875 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_DISCONADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_DISCONADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_DISCONADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_DISCONADJ {
        RX_DISCONADJ::from_bits(val)
    }
}
impl From<RX_DISCONADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_DISCONADJ) -> u8 {
        RX_DISCONADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_ENVADJ {
    #[doc = "Trip-Level Voltage is 0.1000 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_ENVADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_ENVADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_ENVADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_ENVADJ {
        RX_ENVADJ::from_bits(val)
    }
}
impl From<RX_ENVADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_ENVADJ) -> u8 {
        RX_ENVADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_RXDBYPASS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver."]
    value1 = 0x01,
}
impl RX_RXDBYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_RXDBYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_RXDBYPASS {
    #[inline(always)]
    fn from(val: u8) -> RX_RXDBYPASS {
        RX_RXDBYPASS::from_bits(val)
    }
}
impl From<RX_RXDBYPASS> for u8 {
    #[inline(always)]
    fn from(val: RX_RXDBYPASS) -> u8 {
        RX_RXDBYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_SET_DISCONADJ {
    #[doc = "Trip-Level Voltage is 0.56875 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_SET_DISCONADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_SET_DISCONADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_SET_DISCONADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_SET_DISCONADJ {
        RX_SET_DISCONADJ::from_bits(val)
    }
}
impl From<RX_SET_DISCONADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_SET_DISCONADJ) -> u8 {
        RX_SET_DISCONADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_SET_ENVADJ {
    #[doc = "Trip-Level Voltage is 0.1000 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_SET_ENVADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_SET_ENVADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_SET_ENVADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_SET_ENVADJ {
        RX_SET_ENVADJ::from_bits(val)
    }
}
impl From<RX_SET_ENVADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_SET_ENVADJ) -> u8 {
        RX_SET_ENVADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_SET_RXDBYPASS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver."]
    value1 = 0x01,
}
impl RX_SET_RXDBYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_SET_RXDBYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_SET_RXDBYPASS {
    #[inline(always)]
    fn from(val: u8) -> RX_SET_RXDBYPASS {
        RX_SET_RXDBYPASS::from_bits(val)
    }
}
impl From<RX_SET_RXDBYPASS> for u8 {
    #[inline(always)]
    fn from(val: RX_SET_RXDBYPASS) -> u8 {
        RX_SET_RXDBYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_TOG_DISCONADJ {
    #[doc = "Trip-Level Voltage is 0.56875 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_TOG_DISCONADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_TOG_DISCONADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_TOG_DISCONADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_TOG_DISCONADJ {
        RX_TOG_DISCONADJ::from_bits(val)
    }
}
impl From<RX_TOG_DISCONADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_TOG_DISCONADJ) -> u8 {
        RX_TOG_DISCONADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_TOG_ENVADJ {
    #[doc = "Trip-Level Voltage is 0.1000 V."]
    value0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V."]
    value1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V."]
    value2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V."]
    value3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RX_TOG_ENVADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_TOG_ENVADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_TOG_ENVADJ {
    #[inline(always)]
    fn from(val: u8) -> RX_TOG_ENVADJ {
        RX_TOG_ENVADJ::from_bits(val)
    }
}
impl From<RX_TOG_ENVADJ> for u8 {
    #[inline(always)]
    fn from(val: RX_TOG_ENVADJ) -> u8 {
        RX_TOG_ENVADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RX_TOG_RXDBYPASS {
    #[doc = "Normal operation."]
    value0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver."]
    value1 = 0x01,
}
impl RX_TOG_RXDBYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RX_TOG_RXDBYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RX_TOG_RXDBYPASS {
    #[inline(always)]
    fn from(val: u8) -> RX_TOG_RXDBYPASS {
        RX_TOG_RXDBYPASS::from_bits(val)
    }
}
impl From<RX_TOG_RXDBYPASS> for u8 {
    #[inline(always)]
    fn from(val: RX_TOG_RXDBYPASS) -> u8 {
        RX_TOG_RXDBYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CLR_D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    value7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    value15 = 0x0f,
}
impl TX_CLR_D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CLR_D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CLR_D_CAL {
    #[inline(always)]
    fn from(val: u8) -> TX_CLR_D_CAL {
        TX_CLR_D_CAL::from_bits(val)
    }
}
impl From<TX_CLR_D_CAL> for u8 {
    #[inline(always)]
    fn from(val: TX_CLR_D_CAL) -> u8 {
        TX_CLR_D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    value7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    value15 = 0x0f,
}
impl TX_D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_D_CAL {
    #[inline(always)]
    fn from(val: u8) -> TX_D_CAL {
        TX_D_CAL::from_bits(val)
    }
}
impl From<TX_D_CAL> for u8 {
    #[inline(always)]
    fn from(val: TX_D_CAL) -> u8 {
        TX_D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_SET_D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    value7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    value15 = 0x0f,
}
impl TX_SET_D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_SET_D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_SET_D_CAL {
    #[inline(always)]
    fn from(val: u8) -> TX_SET_D_CAL {
        TX_SET_D_CAL::from_bits(val)
    }
}
impl From<TX_SET_D_CAL> for u8 {
    #[inline(always)]
    fn from(val: TX_SET_D_CAL) -> u8 {
        TX_SET_D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_TOG_D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    value7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    value15 = 0x0f,
}
impl TX_TOG_D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_TOG_D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_TOG_D_CAL {
    #[inline(always)]
    fn from(val: u8) -> TX_TOG_D_CAL {
        TX_TOG_D_CAL::from_bits(val)
    }
}
impl From<TX_TOG_D_CAL> for u8 {
    #[inline(always)]
    fn from(val: TX_TOG_D_CAL) -> u8 {
        TX_TOG_D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
    #[doc = "VBUS discharge resistor is disabled (Default)."]
    value0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
        USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS) -> u8 {
        USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external ID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
        USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
    #[doc = "Select the muxed value chosen using VBUS_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
    #[doc = "Powers down the VBUS_VALID comparator."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)."]
    value1 = 0x07,
}
impl USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
        USB1_VBUS_DETECT_CLR_PWRUP_CMPS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_PWRUP_CMPS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_PWRUP_CMPS) -> u8 {
        USB1_VBUS_DETECT_CLR_PWRUP_CMPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
        USB1_VBUS_DETECT_CLR_VBUSVALID_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_VBUSVALID_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_VBUSVALID_SEL) -> u8 {
        USB1_VBUS_DETECT_CLR_VBUSVALID_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
    #[doc = "4.0V."]
    value0 = 0x0,
    #[doc = "4.1V."]
    value1 = 0x01,
    #[doc = "4.2V."]
    value2 = 0x02,
    #[doc = "4.3V."]
    value3 = 0x03,
    #[doc = "4.4V(Default)."]
    value4 = 0x04,
    #[doc = "4.5V."]
    value5 = 0x05,
    #[doc = "4.6V."]
    value6 = 0x06,
    #[doc = "4.7V."]
    value7 = 0x07,
}
impl USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
        USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH) -> u8 {
        USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results."]
    value0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
        USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID) -> u8 {
        USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)."]
    value0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
        USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL) -> u8 {
        USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_DISCHARGE_VBUS {
    #[doc = "VBUS discharge resistor is disabled (Default)."]
    value0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_DISCHARGE_VBUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_DISCHARGE_VBUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_DISCHARGE_VBUS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_DISCHARGE_VBUS {
        USB1_VBUS_DETECT_DISCHARGE_VBUS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_DISCHARGE_VBUS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_DISCHARGE_VBUS) -> u8 {
        USB1_VBUS_DETECT_DISCHARGE_VBUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external ID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
        USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_PWRUP_CMPS {
    #[doc = "Powers down the VBUS_VALID comparator."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)."]
    value1 = 0x07,
}
impl USB1_VBUS_DETECT_PWRUP_CMPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_PWRUP_CMPS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_PWRUP_CMPS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_PWRUP_CMPS {
        USB1_VBUS_DETECT_PWRUP_CMPS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_PWRUP_CMPS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_PWRUP_CMPS) -> u8 {
        USB1_VBUS_DETECT_PWRUP_CMPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
    #[doc = "VBUS discharge resistor is disabled (Default)."]
    value0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
        USB1_VBUS_DETECT_SET_DISCHARGE_VBUS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_DISCHARGE_VBUS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_DISCHARGE_VBUS) -> u8 {
        USB1_VBUS_DETECT_SET_DISCHARGE_VBUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external ID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
        USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_PWRUP_CMPS {
    #[doc = "Powers down the VBUS_VALID comparator."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)."]
    value1 = 0x07,
}
impl USB1_VBUS_DETECT_SET_PWRUP_CMPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_PWRUP_CMPS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_PWRUP_CMPS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_PWRUP_CMPS {
        USB1_VBUS_DETECT_SET_PWRUP_CMPS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_PWRUP_CMPS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_PWRUP_CMPS) -> u8 {
        USB1_VBUS_DETECT_SET_PWRUP_CMPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
        USB1_VBUS_DETECT_SET_VBUSVALID_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_VBUSVALID_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_VBUSVALID_SEL) -> u8 {
        USB1_VBUS_DETECT_SET_VBUSVALID_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
    #[doc = "4.0V."]
    value0 = 0x0,
    #[doc = "4.1V."]
    value1 = 0x01,
    #[doc = "4.2V."]
    value2 = 0x02,
    #[doc = "4.3V."]
    value3 = 0x03,
    #[doc = "4.4V(Default)."]
    value4 = 0x04,
    #[doc = "4.5V."]
    value5 = 0x05,
    #[doc = "4.6V."]
    value6 = 0x06,
    #[doc = "4.7V."]
    value7 = 0x07,
}
impl USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
        USB1_VBUS_DETECT_SET_VBUSVALID_THRESH::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_VBUSVALID_THRESH> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_VBUSVALID_THRESH) -> u8 {
        USB1_VBUS_DETECT_SET_VBUSVALID_THRESH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results."]
    value0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
        USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID) -> u8 {
        USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)."]
    value0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
        USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL) -> u8 {
        USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
    #[doc = "VBUS discharge resistor is disabled (Default)."]
    value0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
        USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS) -> u8 {
        USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
    #[doc = "Select the muxed value chosen using ID_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external ID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
        USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    value0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
    #[doc = "Powers down the VBUS_VALID comparator."]
    value0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)."]
    value1 = 0x07,
}
impl USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
        USB1_VBUS_DETECT_TOG_PWRUP_CMPS::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_PWRUP_CMPS> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_PWRUP_CMPS) -> u8 {
        USB1_VBUS_DETECT_TOG_PWRUP_CMPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
        USB1_VBUS_DETECT_TOG_VBUSVALID_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_VBUSVALID_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_VBUSVALID_SEL) -> u8 {
        USB1_VBUS_DETECT_TOG_VBUSVALID_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
    #[doc = "4.0V."]
    value0 = 0x0,
    #[doc = "4.1V."]
    value1 = 0x01,
    #[doc = "4.2V."]
    value2 = 0x02,
    #[doc = "4.3V."]
    value3 = 0x03,
    #[doc = "4.4V(Default)."]
    value4 = 0x04,
    #[doc = "4.5V."]
    value5 = 0x05,
    #[doc = "4.6V."]
    value6 = 0x06,
    #[doc = "4.7V."]
    value7 = 0x07,
}
impl USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
        USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH) -> u8 {
        USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results."]
    value0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
        USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID) -> u8 {
        USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)."]
    value0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
        USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL) -> u8 {
        USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_VBUSVALID_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_VBUSVALID_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_VBUSVALID_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_SEL {
        USB1_VBUS_DETECT_VBUSVALID_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_VBUSVALID_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_VBUSVALID_SEL) -> u8 {
        USB1_VBUS_DETECT_VBUSVALID_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_VBUSVALID_THRESH {
    #[doc = "4.0V."]
    value0 = 0x0,
    #[doc = "4.1V."]
    value1 = 0x01,
    #[doc = "4.2V."]
    value2 = 0x02,
    #[doc = "4.3V."]
    value3 = 0x03,
    #[doc = "4.4V(Default)."]
    value4 = 0x04,
    #[doc = "4.5V."]
    value5 = 0x05,
    #[doc = "4.6V."]
    value6 = 0x06,
    #[doc = "4.7V."]
    value7 = 0x07,
}
impl USB1_VBUS_DETECT_VBUSVALID_THRESH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_THRESH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_VBUSVALID_THRESH {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_THRESH {
        USB1_VBUS_DETECT_VBUSVALID_THRESH::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_VBUSVALID_THRESH> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_VBUSVALID_THRESH) -> u8 {
        USB1_VBUS_DETECT_VBUSVALID_THRESH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results."]
    value0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
        USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID) -> u8 {
        USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)."]
    value0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    value1 = 0x01,
}
impl USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
        USB1_VBUS_DETECT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_VBUS_OVERRIDE_EN) -> u8 {
        USB1_VBUS_DETECT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)."]
    value0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller."]
    value2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
        USB1_VBUS_DETECT_VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<USB1_VBUS_DETECT_VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: USB1_VBUS_DETECT_VBUS_SOURCE_SEL) -> u8 {
        USB1_VBUS_DETECT_VBUS_SOURCE_SEL::to_bits(val)
    }
}
