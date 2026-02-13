#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlClrDevPulldown {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 0x01,
}
impl AnactrlClrDevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnactrlClrDevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnactrlClrDevPulldown {
    #[inline(always)]
    fn from(val: u8) -> AnactrlClrDevPulldown {
        AnactrlClrDevPulldown::from_bits(val)
    }
}
impl From<AnactrlClrDevPulldown> for u8 {
    #[inline(always)]
    fn from(val: AnactrlClrDevPulldown) -> u8 {
        AnactrlClrDevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlDevPulldown {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 0x01,
}
impl AnactrlDevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnactrlDevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnactrlDevPulldown {
    #[inline(always)]
    fn from(val: u8) -> AnactrlDevPulldown {
        AnactrlDevPulldown::from_bits(val)
    }
}
impl From<AnactrlDevPulldown> for u8 {
    #[inline(always)]
    fn from(val: AnactrlDevPulldown) -> u8 {
        AnactrlDevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlSetDevPulldown {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 0x01,
}
impl AnactrlSetDevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnactrlSetDevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnactrlSetDevPulldown {
    #[inline(always)]
    fn from(val: u8) -> AnactrlSetDevPulldown {
        AnactrlSetDevPulldown::from_bits(val)
    }
}
impl From<AnactrlSetDevPulldown> for u8 {
    #[inline(always)]
    fn from(val: AnactrlSetDevPulldown) -> u8 {
        AnactrlSetDevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlTogDevPulldown {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 0x01,
}
impl AnactrlTogDevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AnactrlTogDevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AnactrlTogDevPulldown {
    #[inline(always)]
    fn from(val: u8) -> AnactrlTogDevPulldown {
        AnactrlTogDevPulldown::from_bits(val)
    }
}
impl From<AnactrlTogDevPulldown> for u8 {
    #[inline(always)]
    fn from(val: AnactrlTogDevPulldown) -> u8 {
        AnactrlTogDevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrEndevplugindet {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 0x01,
}
impl CtrlClrEndevplugindet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrEndevplugindet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrEndevplugindet {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrEndevplugindet {
        CtrlClrEndevplugindet::from_bits(val)
    }
}
impl From<CtrlClrEndevplugindet> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrEndevplugindet) -> u8 {
        CtrlClrEndevplugindet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlEndevplugindet {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 0x01,
}
impl CtrlEndevplugindet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlEndevplugindet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlEndevplugindet {
    #[inline(always)]
    fn from(val: u8) -> CtrlEndevplugindet {
        CtrlEndevplugindet::from_bits(val)
    }
}
impl From<CtrlEndevplugindet> for u8 {
    #[inline(always)]
    fn from(val: CtrlEndevplugindet) -> u8 {
        CtrlEndevplugindet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetEndevplugindet {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 0x01,
}
impl CtrlSetEndevplugindet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetEndevplugindet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetEndevplugindet {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetEndevplugindet {
        CtrlSetEndevplugindet::from_bits(val)
    }
}
impl From<CtrlSetEndevplugindet> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetEndevplugindet) -> u8 {
        CtrlSetEndevplugindet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogEndevplugindet {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 0x01,
}
impl CtrlTogEndevplugindet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogEndevplugindet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogEndevplugindet {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogEndevplugindet {
        CtrlTogEndevplugindet::from_bits(val)
    }
}
impl From<CtrlTogEndevplugindet> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogEndevplugindet) -> u8 {
        CtrlTogEndevplugindet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginStatus {
    #[doc = "No attachment to a USB host is detected"]
    VALUE0 = 0x0,
    #[doc = "Cable attachment to a USB host is detected"]
    VALUE1 = 0x01,
}
impl DevpluginStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevpluginStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevpluginStatus {
    #[inline(always)]
    fn from(val: u8) -> DevpluginStatus {
        DevpluginStatus::from_bits(val)
    }
}
impl From<DevpluginStatus> for u8 {
    #[inline(always)]
    fn from(val: DevpluginStatus) -> u8 {
        DevpluginStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostdiscondetectStatus {
    #[doc = "USB cable disconnect has not been detected at the local host"]
    VALUE0 = 0x0,
    #[doc = "USB cable disconnect has been detected at the local host"]
    VALUE1 = 0x01,
}
impl HostdiscondetectStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostdiscondetectStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostdiscondetectStatus {
    #[inline(always)]
    fn from(val: u8) -> HostdiscondetectStatus {
        HostdiscondetectStatus::from_bits(val)
    }
}
impl From<HostdiscondetectStatus> for u8 {
    #[inline(always)]
    fn from(val: HostdiscondetectStatus) -> u8 {
        HostdiscondetectStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicClrPllDivSel {
    #[doc = "Divide by 13"]
    VALUE0 = 0x0,
    #[doc = "Divide by 15"]
    VALUE1 = 0x01,
    #[doc = "Divide by 16"]
    VALUE2 = 0x02,
    #[doc = "Divide by 20"]
    VALUE3 = 0x03,
    #[doc = "Divide by 22"]
    VALUE4 = 0x04,
    #[doc = "Divide by 25"]
    VALUE5 = 0x05,
    #[doc = "Divide by 30"]
    VALUE6 = 0x06,
    #[doc = "Divide by 240"]
    VALUE7 = 0x07,
}
impl PllSicClrPllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicClrPllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicClrPllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicClrPllDivSel {
        PllSicClrPllDivSel::from_bits(val)
    }
}
impl From<PllSicClrPllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicClrPllDivSel) -> u8 {
        PllSicClrPllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicClrPllLock {
    #[doc = "PLL is not currently locked"]
    VALUE0 = 0x0,
    #[doc = "PLL is currently locked"]
    VALUE1 = 0x01,
}
impl PllSicClrPllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicClrPllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicClrPllLock {
    #[inline(always)]
    fn from(val: u8) -> PllSicClrPllLock {
        PllSicClrPllLock::from_bits(val)
    }
}
impl From<PllSicClrPllLock> for u8 {
    #[inline(always)]
    fn from(val: PllSicClrPllLock) -> u8 {
        PllSicClrPllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicClrRefbiasPwdSel {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 0x01,
}
impl PllSicClrRefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicClrRefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicClrRefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicClrRefbiasPwdSel {
        PllSicClrRefbiasPwdSel::from_bits(val)
    }
}
impl From<PllSicClrRefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicClrRefbiasPwdSel) -> u8 {
        PllSicClrRefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicPllDivSel {
    #[doc = "Divide by 13"]
    VALUE0 = 0x0,
    #[doc = "Divide by 15"]
    VALUE1 = 0x01,
    #[doc = "Divide by 16"]
    VALUE2 = 0x02,
    #[doc = "Divide by 20"]
    VALUE3 = 0x03,
    #[doc = "Divide by 22"]
    VALUE4 = 0x04,
    #[doc = "Divide by 25"]
    VALUE5 = 0x05,
    #[doc = "Divide by 30"]
    VALUE6 = 0x06,
    #[doc = "Divide by 240"]
    VALUE7 = 0x07,
}
impl PllSicPllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicPllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicPllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicPllDivSel {
        PllSicPllDivSel::from_bits(val)
    }
}
impl From<PllSicPllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicPllDivSel) -> u8 {
        PllSicPllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicPllLock {
    #[doc = "PLL is not currently locked"]
    VALUE0 = 0x0,
    #[doc = "PLL is currently locked"]
    VALUE1 = 0x01,
}
impl PllSicPllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicPllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicPllLock {
    #[inline(always)]
    fn from(val: u8) -> PllSicPllLock {
        PllSicPllLock::from_bits(val)
    }
}
impl From<PllSicPllLock> for u8 {
    #[inline(always)]
    fn from(val: PllSicPllLock) -> u8 {
        PllSicPllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicRefbiasPwdSel {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 0x01,
}
impl PllSicRefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicRefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicRefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicRefbiasPwdSel {
        PllSicRefbiasPwdSel::from_bits(val)
    }
}
impl From<PllSicRefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicRefbiasPwdSel) -> u8 {
        PllSicRefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicSetPllDivSel {
    #[doc = "Divide by 13"]
    VALUE0 = 0x0,
    #[doc = "Divide by 15"]
    VALUE1 = 0x01,
    #[doc = "Divide by 16"]
    VALUE2 = 0x02,
    #[doc = "Divide by 20"]
    VALUE3 = 0x03,
    #[doc = "Divide by 22"]
    VALUE4 = 0x04,
    #[doc = "Divide by 25"]
    VALUE5 = 0x05,
    #[doc = "Divide by 30"]
    VALUE6 = 0x06,
    #[doc = "Divide by 240"]
    VALUE7 = 0x07,
}
impl PllSicSetPllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicSetPllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicSetPllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicSetPllDivSel {
        PllSicSetPllDivSel::from_bits(val)
    }
}
impl From<PllSicSetPllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicSetPllDivSel) -> u8 {
        PllSicSetPllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicSetPllLock {
    #[doc = "PLL is not currently locked"]
    VALUE0 = 0x0,
    #[doc = "PLL is currently locked"]
    VALUE1 = 0x01,
}
impl PllSicSetPllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicSetPllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicSetPllLock {
    #[inline(always)]
    fn from(val: u8) -> PllSicSetPllLock {
        PllSicSetPllLock::from_bits(val)
    }
}
impl From<PllSicSetPllLock> for u8 {
    #[inline(always)]
    fn from(val: PllSicSetPllLock) -> u8 {
        PllSicSetPllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicSetRefbiasPwdSel {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 0x01,
}
impl PllSicSetRefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicSetRefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicSetRefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicSetRefbiasPwdSel {
        PllSicSetRefbiasPwdSel::from_bits(val)
    }
}
impl From<PllSicSetRefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicSetRefbiasPwdSel) -> u8 {
        PllSicSetRefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicTogPllDivSel {
    #[doc = "Divide by 13"]
    VALUE0 = 0x0,
    #[doc = "Divide by 15"]
    VALUE1 = 0x01,
    #[doc = "Divide by 16"]
    VALUE2 = 0x02,
    #[doc = "Divide by 20"]
    VALUE3 = 0x03,
    #[doc = "Divide by 22"]
    VALUE4 = 0x04,
    #[doc = "Divide by 25"]
    VALUE5 = 0x05,
    #[doc = "Divide by 30"]
    VALUE6 = 0x06,
    #[doc = "Divide by 240"]
    VALUE7 = 0x07,
}
impl PllSicTogPllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicTogPllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicTogPllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicTogPllDivSel {
        PllSicTogPllDivSel::from_bits(val)
    }
}
impl From<PllSicTogPllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicTogPllDivSel) -> u8 {
        PllSicTogPllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicTogPllLock {
    #[doc = "PLL is not currently locked"]
    VALUE0 = 0x0,
    #[doc = "PLL is currently locked"]
    VALUE1 = 0x01,
}
impl PllSicTogPllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicTogPllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicTogPllLock {
    #[inline(always)]
    fn from(val: u8) -> PllSicTogPllLock {
        PllSicTogPllLock::from_bits(val)
    }
}
impl From<PllSicTogPllLock> for u8 {
    #[inline(always)]
    fn from(val: PllSicTogPllLock) -> u8 {
        PllSicTogPllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllSicTogRefbiasPwdSel {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 0x01,
}
impl PllSicTogRefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllSicTogRefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllSicTogRefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> PllSicTogRefbiasPwdSel {
        PllSicTogRefbiasPwdSel::from_bits(val)
    }
}
impl From<PllSicTogRefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: PllSicTogRefbiasPwdSel) -> u8 {
        PllSicTogRefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrRxpwd1pt1 {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1 = 0x01,
}
impl PwdClrRxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrRxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrRxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> PwdClrRxpwd1pt1 {
        PwdClrRxpwd1pt1::from_bits(val)
    }
}
impl From<PwdClrRxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: PwdClrRxpwd1pt1) -> u8 {
        PwdClrRxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrRxpwddiff {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1 = 0x01,
}
impl PwdClrRxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrRxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrRxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> PwdClrRxpwddiff {
        PwdClrRxpwddiff::from_bits(val)
    }
}
impl From<PwdClrRxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: PwdClrRxpwddiff) -> u8 {
        PwdClrRxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrRxpwdenv {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 0x01,
}
impl PwdClrRxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrRxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrRxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> PwdClrRxpwdenv {
        PwdClrRxpwdenv::from_bits(val)
    }
}
impl From<PwdClrRxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: PwdClrRxpwdenv) -> u8 {
        PwdClrRxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrRxpwdrx {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl PwdClrRxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrRxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrRxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> PwdClrRxpwdrx {
        PwdClrRxpwdrx::from_bits(val)
    }
}
impl From<PwdClrRxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: PwdClrRxpwdrx) -> u8 {
        PwdClrRxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrTxpwdfs {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 0x01,
}
impl PwdClrTxpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrTxpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrTxpwdfs {
    #[inline(always)]
    fn from(val: u8) -> PwdClrTxpwdfs {
        PwdClrTxpwdfs::from_bits(val)
    }
}
impl From<PwdClrTxpwdfs> for u8 {
    #[inline(always)]
    fn from(val: PwdClrTxpwdfs) -> u8 {
        PwdClrTxpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrTxpwdibias {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 0x01,
}
impl PwdClrTxpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrTxpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrTxpwdibias {
    #[inline(always)]
    fn from(val: u8) -> PwdClrTxpwdibias {
        PwdClrTxpwdibias::from_bits(val)
    }
}
impl From<PwdClrTxpwdibias> for u8 {
    #[inline(always)]
    fn from(val: PwdClrTxpwdibias) -> u8 {
        PwdClrTxpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrTxpwdv2i {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 0x01,
}
impl PwdClrTxpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdClrTxpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdClrTxpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> PwdClrTxpwdv2i {
        PwdClrTxpwdv2i::from_bits(val)
    }
}
impl From<PwdClrTxpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: PwdClrTxpwdv2i) -> u8 {
        PwdClrTxpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdRxpwd1pt1 {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1 = 0x01,
}
impl PwdRxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdRxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdRxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> PwdRxpwd1pt1 {
        PwdRxpwd1pt1::from_bits(val)
    }
}
impl From<PwdRxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: PwdRxpwd1pt1) -> u8 {
        PwdRxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdRxpwddiff {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1 = 0x01,
}
impl PwdRxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdRxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdRxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> PwdRxpwddiff {
        PwdRxpwddiff::from_bits(val)
    }
}
impl From<PwdRxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: PwdRxpwddiff) -> u8 {
        PwdRxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdRxpwdenv {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 0x01,
}
impl PwdRxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdRxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdRxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> PwdRxpwdenv {
        PwdRxpwdenv::from_bits(val)
    }
}
impl From<PwdRxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: PwdRxpwdenv) -> u8 {
        PwdRxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdRxpwdrx {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl PwdRxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdRxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdRxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> PwdRxpwdrx {
        PwdRxpwdrx::from_bits(val)
    }
}
impl From<PwdRxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: PwdRxpwdrx) -> u8 {
        PwdRxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetRxpwd1pt1 {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1 = 0x01,
}
impl PwdSetRxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetRxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetRxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> PwdSetRxpwd1pt1 {
        PwdSetRxpwd1pt1::from_bits(val)
    }
}
impl From<PwdSetRxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: PwdSetRxpwd1pt1) -> u8 {
        PwdSetRxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetRxpwddiff {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1 = 0x01,
}
impl PwdSetRxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetRxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetRxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> PwdSetRxpwddiff {
        PwdSetRxpwddiff::from_bits(val)
    }
}
impl From<PwdSetRxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: PwdSetRxpwddiff) -> u8 {
        PwdSetRxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetRxpwdenv {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 0x01,
}
impl PwdSetRxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetRxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetRxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> PwdSetRxpwdenv {
        PwdSetRxpwdenv::from_bits(val)
    }
}
impl From<PwdSetRxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: PwdSetRxpwdenv) -> u8 {
        PwdSetRxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetRxpwdrx {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl PwdSetRxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetRxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetRxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> PwdSetRxpwdrx {
        PwdSetRxpwdrx::from_bits(val)
    }
}
impl From<PwdSetRxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: PwdSetRxpwdrx) -> u8 {
        PwdSetRxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetTxpwdfs {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 0x01,
}
impl PwdSetTxpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetTxpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetTxpwdfs {
    #[inline(always)]
    fn from(val: u8) -> PwdSetTxpwdfs {
        PwdSetTxpwdfs::from_bits(val)
    }
}
impl From<PwdSetTxpwdfs> for u8 {
    #[inline(always)]
    fn from(val: PwdSetTxpwdfs) -> u8 {
        PwdSetTxpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetTxpwdibias {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 0x01,
}
impl PwdSetTxpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetTxpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetTxpwdibias {
    #[inline(always)]
    fn from(val: u8) -> PwdSetTxpwdibias {
        PwdSetTxpwdibias::from_bits(val)
    }
}
impl From<PwdSetTxpwdibias> for u8 {
    #[inline(always)]
    fn from(val: PwdSetTxpwdibias) -> u8 {
        PwdSetTxpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdSetTxpwdv2i {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 0x01,
}
impl PwdSetTxpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdSetTxpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdSetTxpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> PwdSetTxpwdv2i {
        PwdSetTxpwdv2i::from_bits(val)
    }
}
impl From<PwdSetTxpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: PwdSetTxpwdv2i) -> u8 {
        PwdSetTxpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogRxpwd1pt1 {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    VALUE1 = 0x01,
}
impl PwdTogRxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogRxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogRxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> PwdTogRxpwd1pt1 {
        PwdTogRxpwd1pt1::from_bits(val)
    }
}
impl From<PwdTogRxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: PwdTogRxpwd1pt1) -> u8 {
        PwdTogRxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogRxpwddiff {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    VALUE1 = 0x01,
}
impl PwdTogRxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogRxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogRxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> PwdTogRxpwddiff {
        PwdTogRxpwddiff::from_bits(val)
    }
}
impl From<PwdTogRxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: PwdTogRxpwddiff) -> u8 {
        PwdTogRxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogRxpwdenv {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 0x01,
}
impl PwdTogRxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogRxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogRxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> PwdTogRxpwdenv {
        PwdTogRxpwdenv::from_bits(val)
    }
}
impl From<PwdTogRxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: PwdTogRxpwdenv) -> u8 {
        PwdTogRxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogRxpwdrx {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl PwdTogRxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogRxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogRxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> PwdTogRxpwdrx {
        PwdTogRxpwdrx::from_bits(val)
    }
}
impl From<PwdTogRxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: PwdTogRxpwdrx) -> u8 {
        PwdTogRxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogTxpwdfs {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 0x01,
}
impl PwdTogTxpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogTxpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogTxpwdfs {
    #[inline(always)]
    fn from(val: u8) -> PwdTogTxpwdfs {
        PwdTogTxpwdfs::from_bits(val)
    }
}
impl From<PwdTogTxpwdfs> for u8 {
    #[inline(always)]
    fn from(val: PwdTogTxpwdfs) -> u8 {
        PwdTogTxpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogTxpwdibias {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 0x01,
}
impl PwdTogTxpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogTxpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogTxpwdibias {
    #[inline(always)]
    fn from(val: u8) -> PwdTogTxpwdibias {
        PwdTogTxpwdibias::from_bits(val)
    }
}
impl From<PwdTogTxpwdibias> for u8 {
    #[inline(always)]
    fn from(val: PwdTogTxpwdibias) -> u8 {
        PwdTogTxpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTogTxpwdv2i {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 0x01,
}
impl PwdTogTxpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTogTxpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTogTxpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> PwdTogTxpwdv2i {
        PwdTogTxpwdv2i::from_bits(val)
    }
}
impl From<PwdTogTxpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: PwdTogTxpwdv2i) -> u8 {
        PwdTogTxpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTxpwdfs {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 0x01,
}
impl PwdTxpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTxpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTxpwdfs {
    #[inline(always)]
    fn from(val: u8) -> PwdTxpwdfs {
        PwdTxpwdfs::from_bits(val)
    }
}
impl From<PwdTxpwdfs> for u8 {
    #[inline(always)]
    fn from(val: PwdTxpwdfs) -> u8 {
        PwdTxpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTxpwdibias {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 0x01,
}
impl PwdTxpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTxpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTxpwdibias {
    #[inline(always)]
    fn from(val: u8) -> PwdTxpwdibias {
        PwdTxpwdibias::from_bits(val)
    }
}
impl From<PwdTxpwdibias> for u8 {
    #[inline(always)]
    fn from(val: PwdTxpwdibias) -> u8 {
        PwdTxpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdTxpwdv2i {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 0x01,
}
impl PwdTxpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdTxpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdTxpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> PwdTxpwdv2i {
        PwdTxpwdv2i::from_bits(val)
    }
}
impl From<PwdTxpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: PwdTxpwdv2i) -> u8 {
        PwdTxpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClrDisconadj {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxClrDisconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClrDisconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClrDisconadj {
    #[inline(always)]
    fn from(val: u8) -> RxClrDisconadj {
        RxClrDisconadj::from_bits(val)
    }
}
impl From<RxClrDisconadj> for u8 {
    #[inline(always)]
    fn from(val: RxClrDisconadj) -> u8 {
        RxClrDisconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClrEnvadj {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxClrEnvadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClrEnvadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClrEnvadj {
    #[inline(always)]
    fn from(val: u8) -> RxClrEnvadj {
        RxClrEnvadj::from_bits(val)
    }
}
impl From<RxClrEnvadj> for u8 {
    #[inline(always)]
    fn from(val: RxClrEnvadj) -> u8 {
        RxClrEnvadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClrRxdbypass {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl RxClrRxdbypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClrRxdbypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClrRxdbypass {
    #[inline(always)]
    fn from(val: u8) -> RxClrRxdbypass {
        RxClrRxdbypass::from_bits(val)
    }
}
impl From<RxClrRxdbypass> for u8 {
    #[inline(always)]
    fn from(val: RxClrRxdbypass) -> u8 {
        RxClrRxdbypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxDisconadj {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxDisconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxDisconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxDisconadj {
    #[inline(always)]
    fn from(val: u8) -> RxDisconadj {
        RxDisconadj::from_bits(val)
    }
}
impl From<RxDisconadj> for u8 {
    #[inline(always)]
    fn from(val: RxDisconadj) -> u8 {
        RxDisconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxEnvadj {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxEnvadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxEnvadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxEnvadj {
    #[inline(always)]
    fn from(val: u8) -> RxEnvadj {
        RxEnvadj::from_bits(val)
    }
}
impl From<RxEnvadj> for u8 {
    #[inline(always)]
    fn from(val: RxEnvadj) -> u8 {
        RxEnvadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxRxdbypass {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl RxRxdbypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxRxdbypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxRxdbypass {
    #[inline(always)]
    fn from(val: u8) -> RxRxdbypass {
        RxRxdbypass::from_bits(val)
    }
}
impl From<RxRxdbypass> for u8 {
    #[inline(always)]
    fn from(val: RxRxdbypass) -> u8 {
        RxRxdbypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxSetDisconadj {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxSetDisconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxSetDisconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxSetDisconadj {
    #[inline(always)]
    fn from(val: u8) -> RxSetDisconadj {
        RxSetDisconadj::from_bits(val)
    }
}
impl From<RxSetDisconadj> for u8 {
    #[inline(always)]
    fn from(val: RxSetDisconadj) -> u8 {
        RxSetDisconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxSetEnvadj {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxSetEnvadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxSetEnvadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxSetEnvadj {
    #[inline(always)]
    fn from(val: u8) -> RxSetEnvadj {
        RxSetEnvadj::from_bits(val)
    }
}
impl From<RxSetEnvadj> for u8 {
    #[inline(always)]
    fn from(val: RxSetEnvadj) -> u8 {
        RxSetEnvadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxSetRxdbypass {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl RxSetRxdbypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxSetRxdbypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxSetRxdbypass {
    #[inline(always)]
    fn from(val: u8) -> RxSetRxdbypass {
        RxSetRxdbypass::from_bits(val)
    }
}
impl From<RxSetRxdbypass> for u8 {
    #[inline(always)]
    fn from(val: RxSetRxdbypass) -> u8 {
        RxSetRxdbypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxTogDisconadj {
    #[doc = "Trip-Level Voltage is 0.56875 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxTogDisconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxTogDisconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxTogDisconadj {
    #[inline(always)]
    fn from(val: u8) -> RxTogDisconadj {
        RxTogDisconadj::from_bits(val)
    }
}
impl From<RxTogDisconadj> for u8 {
    #[inline(always)]
    fn from(val: RxTogDisconadj) -> u8 {
        RxTogDisconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxTogEnvadj {
    #[doc = "Trip-Level Voltage is 0.1000 V"]
    VALUE0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    VALUE1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    VALUE2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    VALUE3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RxTogEnvadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxTogEnvadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxTogEnvadj {
    #[inline(always)]
    fn from(val: u8) -> RxTogEnvadj {
        RxTogEnvadj::from_bits(val)
    }
}
impl From<RxTogEnvadj> for u8 {
    #[inline(always)]
    fn from(val: RxTogEnvadj) -> u8 {
        RxTogEnvadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxTogRxdbypass {
    #[doc = "Normal operation."]
    VALUE0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    VALUE1 = 0x01,
}
impl RxTogRxdbypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxTogRxdbypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxTogRxdbypass {
    #[inline(always)]
    fn from(val: u8) -> RxTogRxdbypass {
        RxTogRxdbypass::from_bits(val)
    }
}
impl From<RxTogRxdbypass> for u8 {
    #[inline(always)]
    fn from(val: RxTogRxdbypass) -> u8 {
        RxTogRxdbypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxClrDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    VALUE7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15 = 0x0f,
}
impl TxClrDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxClrDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxClrDCal {
    #[inline(always)]
    fn from(val: u8) -> TxClrDCal {
        TxClrDCal::from_bits(val)
    }
}
impl From<TxClrDCal> for u8 {
    #[inline(always)]
    fn from(val: TxClrDCal) -> u8 {
        TxClrDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    VALUE7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15 = 0x0f,
}
impl TxDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxDCal {
    #[inline(always)]
    fn from(val: u8) -> TxDCal {
        TxDCal::from_bits(val)
    }
}
impl From<TxDCal> for u8 {
    #[inline(always)]
    fn from(val: TxDCal) -> u8 {
        TxDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxSetDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    VALUE7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15 = 0x0f,
}
impl TxSetDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxSetDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxSetDCal {
    #[inline(always)]
    fn from(val: u8) -> TxSetDCal {
        TxSetDCal::from_bits(val)
    }
}
impl From<TxSetDCal> for u8 {
    #[inline(always)]
    fn from(val: TxSetDCal) -> u8 {
        TxSetDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxTogDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    VALUE7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    VALUE15 = 0x0f,
}
impl TxTogDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxTogDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxTogDCal {
    #[inline(always)]
    fn from(val: u8) -> TxTogDCal {
        TxTogDCal::from_bits(val)
    }
}
impl From<TxTogDCal> for u8 {
    #[inline(always)]
    fn from(val: TxTogDCal) -> u8 {
        TxTogDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrDischargeVbus {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrDischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrDischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrDischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrDischargeVbus {
        Usb1VbusDetectClrDischargeVbus::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrDischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrDischargeVbus) -> u8 {
        Usb1VbusDetectClrDischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrExtIdOverrideEn {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external ID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrExtIdOverrideEn {
        Usb1VbusDetectClrExtIdOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrExtIdOverrideEn) -> u8 {
        Usb1VbusDetectClrExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrExtVbusOverrideEn {
    #[doc = "Select the muxed value chosen using VBUS_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrExtVbusOverrideEn {
        Usb1VbusDetectClrExtVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrExtVbusOverrideEn) -> u8 {
        Usb1VbusDetectClrExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1 = 0x07,
}
impl Usb1VbusDetectClrPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrPwrupCmps {
        Usb1VbusDetectClrPwrupCmps::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrPwrupCmps) -> u8 {
        Usb1VbusDetectClrPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusOverrideEn {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusOverrideEn {
        Usb1VbusDetectClrVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusOverrideEn) -> u8 {
        Usb1VbusDetectClrVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusSourceSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb1VbusDetectClrVbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusSourceSel {
        Usb1VbusDetectClrVbusSourceSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusSourceSel) -> u8 {
        Usb1VbusDetectClrVbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusvalidSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrVbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusvalidSel {
        Usb1VbusDetectClrVbusvalidSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusvalidSel) -> u8 {
        Usb1VbusDetectClrVbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusvalidThresh {
    #[doc = "4.0V"]
    VALUE0 = 0x0,
    #[doc = "4.1V"]
    VALUE1 = 0x01,
    #[doc = "4.2V"]
    VALUE2 = 0x02,
    #[doc = "4.3V"]
    VALUE3 = 0x03,
    #[doc = "4.4V(Default)"]
    VALUE4 = 0x04,
    #[doc = "4.5V"]
    VALUE5 = 0x05,
    #[doc = "4.6V"]
    VALUE6 = 0x06,
    #[doc = "4.7V"]
    VALUE7 = 0x07,
}
impl Usb1VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusvalidThresh {
        Usb1VbusDetectClrVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusvalidThresh) -> u8 {
        Usb1VbusDetectClrVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusvalidToSessvalid {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectClrVbusvalidToSessvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusvalidToSessvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusvalidToSessvalid {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusvalidToSessvalid {
        Usb1VbusDetectClrVbusvalidToSessvalid::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusvalidToSessvalid> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusvalidToSessvalid) -> u8 {
        Usb1VbusDetectClrVbusvalidToSessvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectDischargeVbus {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectDischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectDischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectDischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectDischargeVbus {
        Usb1VbusDetectDischargeVbus::from_bits(val)
    }
}
impl From<Usb1VbusDetectDischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectDischargeVbus) -> u8 {
        Usb1VbusDetectDischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectExtIdOverrideEn {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external ID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectExtIdOverrideEn {
        Usb1VbusDetectExtIdOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectExtIdOverrideEn) -> u8 {
        Usb1VbusDetectExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectExtVbusOverrideEn {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectExtVbusOverrideEn {
        Usb1VbusDetectExtVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectExtVbusOverrideEn) -> u8 {
        Usb1VbusDetectExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1 = 0x07,
}
impl Usb1VbusDetectPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectPwrupCmps {
        Usb1VbusDetectPwrupCmps::from_bits(val)
    }
}
impl From<Usb1VbusDetectPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectPwrupCmps) -> u8 {
        Usb1VbusDetectPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetDischargeVbus {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetDischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetDischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetDischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetDischargeVbus {
        Usb1VbusDetectSetDischargeVbus::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetDischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetDischargeVbus) -> u8 {
        Usb1VbusDetectSetDischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetExtIdOverrideEn {
    #[doc = "Select the Muxed value chosen using ID_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external ID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetExtIdOverrideEn {
        Usb1VbusDetectSetExtIdOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetExtIdOverrideEn) -> u8 {
        Usb1VbusDetectSetExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetExtVbusOverrideEn {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetExtVbusOverrideEn {
        Usb1VbusDetectSetExtVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetExtVbusOverrideEn) -> u8 {
        Usb1VbusDetectSetExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1 = 0x07,
}
impl Usb1VbusDetectSetPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetPwrupCmps {
        Usb1VbusDetectSetPwrupCmps::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetPwrupCmps) -> u8 {
        Usb1VbusDetectSetPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusOverrideEn {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusOverrideEn {
        Usb1VbusDetectSetVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusOverrideEn) -> u8 {
        Usb1VbusDetectSetVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusSourceSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb1VbusDetectSetVbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusSourceSel {
        Usb1VbusDetectSetVbusSourceSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusSourceSel) -> u8 {
        Usb1VbusDetectSetVbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusvalidSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetVbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusvalidSel {
        Usb1VbusDetectSetVbusvalidSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusvalidSel) -> u8 {
        Usb1VbusDetectSetVbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusvalidThresh {
    #[doc = "4.0V"]
    VALUE0 = 0x0,
    #[doc = "4.1V"]
    VALUE1 = 0x01,
    #[doc = "4.2V"]
    VALUE2 = 0x02,
    #[doc = "4.3V"]
    VALUE3 = 0x03,
    #[doc = "4.4V(Default)"]
    VALUE4 = 0x04,
    #[doc = "4.5V"]
    VALUE5 = 0x05,
    #[doc = "4.6V"]
    VALUE6 = 0x06,
    #[doc = "4.7V"]
    VALUE7 = 0x07,
}
impl Usb1VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusvalidThresh {
        Usb1VbusDetectSetVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusvalidThresh) -> u8 {
        Usb1VbusDetectSetVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusvalidToSessvalid {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectSetVbusvalidToSessvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusvalidToSessvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusvalidToSessvalid {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusvalidToSessvalid {
        Usb1VbusDetectSetVbusvalidToSessvalid::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusvalidToSessvalid> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusvalidToSessvalid) -> u8 {
        Usb1VbusDetectSetVbusvalidToSessvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogDischargeVbus {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    VALUE0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogDischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogDischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogDischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogDischargeVbus {
        Usb1VbusDetectTogDischargeVbus::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogDischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogDischargeVbus) -> u8 {
        Usb1VbusDetectTogDischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogExtIdOverrideEn {
    #[doc = "Select the muxed value chosen using ID_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external ID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogExtIdOverrideEn {
        Usb1VbusDetectTogExtIdOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogExtIdOverrideEn) -> u8 {
        Usb1VbusDetectTogExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogExtVbusOverrideEn {
    #[doc = "Select the Muxed value chosen using VBUS_OVERRIDE_EN."]
    VALUE0 = 0x0,
    #[doc = "Select the external VBUS VALID value."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogExtVbusOverrideEn {
        Usb1VbusDetectTogExtVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogExtVbusOverrideEn) -> u8 {
        Usb1VbusDetectTogExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    VALUE0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    VALUE1 = 0x07,
}
impl Usb1VbusDetectTogPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogPwrupCmps {
        Usb1VbusDetectTogPwrupCmps::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogPwrupCmps) -> u8 {
        Usb1VbusDetectTogPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusOverrideEn {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusOverrideEn {
        Usb1VbusDetectTogVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusOverrideEn) -> u8 {
        Usb1VbusDetectTogVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusSourceSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb1VbusDetectTogVbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusSourceSel {
        Usb1VbusDetectTogVbusSourceSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusSourceSel) -> u8 {
        Usb1VbusDetectTogVbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusvalidSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogVbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusvalidSel {
        Usb1VbusDetectTogVbusvalidSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusvalidSel) -> u8 {
        Usb1VbusDetectTogVbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusvalidThresh {
    #[doc = "4.0V"]
    VALUE0 = 0x0,
    #[doc = "4.1V"]
    VALUE1 = 0x01,
    #[doc = "4.2V"]
    VALUE2 = 0x02,
    #[doc = "4.3V"]
    VALUE3 = 0x03,
    #[doc = "4.4V(Default)"]
    VALUE4 = 0x04,
    #[doc = "4.5V"]
    VALUE5 = 0x05,
    #[doc = "4.6V"]
    VALUE6 = 0x06,
    #[doc = "4.7V"]
    VALUE7 = 0x07,
}
impl Usb1VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusvalidThresh {
        Usb1VbusDetectTogVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusvalidThresh) -> u8 {
        Usb1VbusDetectTogVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusvalidToSessvalid {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectTogVbusvalidToSessvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusvalidToSessvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusvalidToSessvalid {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusvalidToSessvalid {
        Usb1VbusDetectTogVbusvalidToSessvalid::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusvalidToSessvalid> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusvalidToSessvalid) -> u8 {
        Usb1VbusDetectTogVbusvalidToSessvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusOverrideEn {
    #[doc = "Use the results of the internal VBUS_VALID and Session Valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusOverrideEn {
        Usb1VbusDetectVbusOverrideEn::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusOverrideEn) -> u8 {
        Usb1VbusDetectVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusSourceSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VALUE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb1VbusDetectVbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusSourceSel {
        Usb1VbusDetectVbusSourceSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusSourceSel) -> u8 {
        Usb1VbusDetectVbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusvalidSel {
    #[doc = "Use the VBUS_VALID comparator results for signal reported to the USB controller (Default)"]
    VALUE0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectVbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusvalidSel {
        Usb1VbusDetectVbusvalidSel::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusvalidSel) -> u8 {
        Usb1VbusDetectVbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusvalidThresh {
    #[doc = "4.0V"]
    VALUE0 = 0x0,
    #[doc = "4.1V"]
    VALUE1 = 0x01,
    #[doc = "4.2V"]
    VALUE2 = 0x02,
    #[doc = "4.3V"]
    VALUE3 = 0x03,
    #[doc = "4.4V(Default)"]
    VALUE4 = 0x04,
    #[doc = "4.5V"]
    VALUE5 = 0x05,
    #[doc = "4.6V"]
    VALUE6 = 0x06,
    #[doc = "4.7V"]
    VALUE7 = 0x07,
}
impl Usb1VbusDetectVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusvalidThresh {
        Usb1VbusDetectVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusvalidThresh) -> u8 {
        Usb1VbusDetectVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusvalidToSessvalid {
    #[doc = "Use the VBUS_VALID comparator for VBUS_VALID results"]
    VALUE0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VALUE1 = 0x01,
}
impl Usb1VbusDetectVbusvalidToSessvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusvalidToSessvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusvalidToSessvalid {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusvalidToSessvalid {
        Usb1VbusDetectVbusvalidToSessvalid::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusvalidToSessvalid> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusvalidToSessvalid) -> u8 {
        Usb1VbusDetectVbusvalidToSessvalid::to_bits(val)
    }
}
