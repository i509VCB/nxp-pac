#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnactrlClrDevPulldown {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    DEV_PULLDOWN_0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    DEV_PULLDOWN_1 = 0x01,
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
    DEV_PULLDOWN_0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    DEV_PULLDOWN_1 = 0x01,
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
    DEV_PULLDOWN_0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    DEV_PULLDOWN_1 = 0x01,
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
    DEV_PULLDOWN_0 = 0x0,
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    DEV_PULLDOWN_1 = 0x01,
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
pub enum Avalid {
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    AVALID_0 = 0x0,
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    AVALID_1 = 0x01,
}
impl Avalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avalid {
    #[inline(always)]
    fn from(val: u8) -> Avalid {
        Avalid::from_bits(val)
    }
}
impl From<Avalid> for u8 {
    #[inline(always)]
    fn from(val: Avalid) -> u8 {
        Avalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bvalid {
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    BVALID_0 = 0x0,
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    BVALID_1 = 0x01,
}
impl Bvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bvalid {
    #[inline(always)]
    fn from(val: u8) -> Bvalid {
        Bvalid::from_bits(val)
    }
}
impl From<Bvalid> for u8 {
    #[inline(always)]
    fn from(val: Bvalid) -> u8 {
        Bvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChrgDetected {
    #[doc = "Standard Downstream Port (SDP) has been detected"]
    CHRG_DETECTED_0 = 0x0,
    #[doc = "Charging Port has been detected"]
    CHRG_DETECTED_1 = 0x01,
}
impl ChrgDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChrgDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChrgDetected {
    #[inline(always)]
    fn from(val: u8) -> ChrgDetected {
        ChrgDetected::from_bits(val)
    }
}
impl From<ChrgDetected> for u8 {
    #[inline(always)]
    fn from(val: ChrgDetected) -> u8 {
        ChrgDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrEndevplugindet {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    ENDEVPLUGINDET_0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    ENDEVPLUGINDET_1 = 0x01,
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
    ENDEVPLUGINDET_0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    ENDEVPLUGINDET_1 = 0x01,
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
    ENDEVPLUGINDET_0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    ENDEVPLUGINDET_1 = 0x01,
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
    ENDEVPLUGINDET_0 = 0x0,
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    ENDEVPLUGINDET_1 = 0x01,
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
pub enum Debug1ClrEntailadjvd {
    #[doc = "Delay is nominal"]
    ENTAILADJVD_0 = 0x0,
    #[doc = "Delay is +20%"]
    ENTAILADJVD_1 = 0x01,
    #[doc = "Delay is -20%"]
    ENTAILADJVD_2 = 0x02,
    #[doc = "Delay is -40%"]
    ENTAILADJVD_3 = 0x03,
}
impl Debug1ClrEntailadjvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debug1ClrEntailadjvd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debug1ClrEntailadjvd {
    #[inline(always)]
    fn from(val: u8) -> Debug1ClrEntailadjvd {
        Debug1ClrEntailadjvd::from_bits(val)
    }
}
impl From<Debug1ClrEntailadjvd> for u8 {
    #[inline(always)]
    fn from(val: Debug1ClrEntailadjvd) -> u8 {
        Debug1ClrEntailadjvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debug1Entailadjvd {
    #[doc = "Delay is nominal"]
    ENTAILADJVD_0 = 0x0,
    #[doc = "Delay is +20%"]
    ENTAILADJVD_1 = 0x01,
    #[doc = "Delay is -20%"]
    ENTAILADJVD_2 = 0x02,
    #[doc = "Delay is -40%"]
    ENTAILADJVD_3 = 0x03,
}
impl Debug1Entailadjvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debug1Entailadjvd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debug1Entailadjvd {
    #[inline(always)]
    fn from(val: u8) -> Debug1Entailadjvd {
        Debug1Entailadjvd::from_bits(val)
    }
}
impl From<Debug1Entailadjvd> for u8 {
    #[inline(always)]
    fn from(val: Debug1Entailadjvd) -> u8 {
        Debug1Entailadjvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debug1SetEntailadjvd {
    #[doc = "Delay is nominal"]
    ENTAILADJVD_0 = 0x0,
    #[doc = "Delay is +20%"]
    ENTAILADJVD_1 = 0x01,
    #[doc = "Delay is -20%"]
    ENTAILADJVD_2 = 0x02,
    #[doc = "Delay is -40%"]
    ENTAILADJVD_3 = 0x03,
}
impl Debug1SetEntailadjvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debug1SetEntailadjvd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debug1SetEntailadjvd {
    #[inline(always)]
    fn from(val: u8) -> Debug1SetEntailadjvd {
        Debug1SetEntailadjvd::from_bits(val)
    }
}
impl From<Debug1SetEntailadjvd> for u8 {
    #[inline(always)]
    fn from(val: Debug1SetEntailadjvd) -> u8 {
        Debug1SetEntailadjvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debug1TogEntailadjvd {
    #[doc = "Delay is nominal"]
    ENTAILADJVD_0 = 0x0,
    #[doc = "Delay is +20%"]
    ENTAILADJVD_1 = 0x01,
    #[doc = "Delay is -20%"]
    ENTAILADJVD_2 = 0x02,
    #[doc = "Delay is -40%"]
    ENTAILADJVD_3 = 0x03,
}
impl Debug1TogEntailadjvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debug1TogEntailadjvd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debug1TogEntailadjvd {
    #[inline(always)]
    fn from(val: u8) -> Debug1TogEntailadjvd {
        Debug1TogEntailadjvd::from_bits(val)
    }
}
impl From<Debug1TogEntailadjvd> for u8 {
    #[inline(always)]
    fn from(val: Debug1TogEntailadjvd) -> u8 {
        Debug1TogEntailadjvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginStatus {
    #[doc = "No attachment to a USB host is detected"]
    DEVPLUGIN_STATUS_0 = 0x0,
    #[doc = "Cable attachment to a USB host is detected"]
    DEVPLUGIN_STATUS_1 = 0x01,
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
pub enum DmState {
    #[doc = "USB_DM pin voltage is < 0.8V"]
    DM_STATE_0 = 0x0,
    #[doc = "USB_DM pin voltage is > 2.0V"]
    DM_STATE_1 = 0x01,
}
impl DmState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmState {
    #[inline(always)]
    fn from(val: u8) -> DmState {
        DmState::from_bits(val)
    }
}
impl From<DmState> for u8 {
    #[inline(always)]
    fn from(val: DmState) -> u8 {
        DmState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpState {
    #[doc = "USB_DP pin voltage is < 0.8V"]
    DP_STATE_0 = 0x0,
    #[doc = "USB_DP pin voltage is > 2.0V"]
    DP_STATE_1 = 0x01,
}
impl DpState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpState {
    #[inline(always)]
    fn from(val: u8) -> DpState {
        DpState::from_bits(val)
    }
}
impl From<DpState> for u8 {
    #[inline(always)]
    fn from(val: DpState) -> u8 {
        DpState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostdiscondetectStatus {
    #[doc = "USB cable disconnect has not been detected at the local host"]
    HOSTDISCONDETECT_STATUS_0 = 0x0,
    #[doc = "USB cable disconnect has been detected at the local host"]
    HOSTDISCONDETECT_STATUS_1 = 0x01,
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
    PLL_DIV_SEL_0 = 0x0,
    #[doc = "Divide by 15"]
    PLL_DIV_SEL_1 = 0x01,
    #[doc = "Divide by 16"]
    PLL_DIV_SEL_2 = 0x02,
    #[doc = "Divide by 20"]
    PLL_DIV_SEL_3 = 0x03,
    #[doc = "Divide by 22"]
    PLL_DIV_SEL_4 = 0x04,
    #[doc = "Divide by 25"]
    PLL_DIV_SEL_5 = 0x05,
    #[doc = "Divide by 30"]
    PLL_DIV_SEL_6 = 0x06,
    #[doc = "Divide by 240"]
    PLL_DIV_SEL_7 = 0x07,
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
    PLL_LOCK_0 = 0x0,
    #[doc = "PLL is currently locked"]
    PLL_LOCK_1 = 0x01,
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
    REFBIAS_PWD_SEL_0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    REFBIAS_PWD_SEL_1 = 0x01,
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
    PLL_DIV_SEL_0 = 0x0,
    #[doc = "Divide by 15"]
    PLL_DIV_SEL_1 = 0x01,
    #[doc = "Divide by 16"]
    PLL_DIV_SEL_2 = 0x02,
    #[doc = "Divide by 20"]
    PLL_DIV_SEL_3 = 0x03,
    #[doc = "Divide by 22"]
    PLL_DIV_SEL_4 = 0x04,
    #[doc = "Divide by 25"]
    PLL_DIV_SEL_5 = 0x05,
    #[doc = "Divide by 30"]
    PLL_DIV_SEL_6 = 0x06,
    #[doc = "Divide by 240"]
    PLL_DIV_SEL_7 = 0x07,
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
    PLL_LOCK_0 = 0x0,
    #[doc = "PLL is currently locked"]
    PLL_LOCK_1 = 0x01,
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
    REFBIAS_PWD_SEL_0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    REFBIAS_PWD_SEL_1 = 0x01,
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
    PLL_DIV_SEL_0 = 0x0,
    #[doc = "Divide by 15"]
    PLL_DIV_SEL_1 = 0x01,
    #[doc = "Divide by 16"]
    PLL_DIV_SEL_2 = 0x02,
    #[doc = "Divide by 20"]
    PLL_DIV_SEL_3 = 0x03,
    #[doc = "Divide by 22"]
    PLL_DIV_SEL_4 = 0x04,
    #[doc = "Divide by 25"]
    PLL_DIV_SEL_5 = 0x05,
    #[doc = "Divide by 30"]
    PLL_DIV_SEL_6 = 0x06,
    #[doc = "Divide by 240"]
    PLL_DIV_SEL_7 = 0x07,
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
    PLL_LOCK_0 = 0x0,
    #[doc = "PLL is currently locked"]
    PLL_LOCK_1 = 0x01,
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
    REFBIAS_PWD_SEL_0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    REFBIAS_PWD_SEL_1 = 0x01,
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
    PLL_DIV_SEL_0 = 0x0,
    #[doc = "Divide by 15"]
    PLL_DIV_SEL_1 = 0x01,
    #[doc = "Divide by 16"]
    PLL_DIV_SEL_2 = 0x02,
    #[doc = "Divide by 20"]
    PLL_DIV_SEL_3 = 0x03,
    #[doc = "Divide by 22"]
    PLL_DIV_SEL_4 = 0x04,
    #[doc = "Divide by 25"]
    PLL_DIV_SEL_5 = 0x05,
    #[doc = "Divide by 30"]
    PLL_DIV_SEL_6 = 0x06,
    #[doc = "Divide by 240"]
    PLL_DIV_SEL_7 = 0x07,
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
    PLL_LOCK_0 = 0x0,
    #[doc = "PLL is currently locked"]
    PLL_LOCK_1 = 0x01,
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
    REFBIAS_PWD_SEL_0 = 0x0,
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    REFBIAS_PWD_SEL_1 = 0x01,
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
pub enum PlugContact {
    #[doc = "No USB cable attachment has been detected"]
    PLUG_CONTACT_0 = 0x0,
    #[doc = "A USB cable attachment between the device and host has been detected"]
    PLUG_CONTACT_1 = 0x01,
}
impl PlugContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlugContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlugContact {
    #[inline(always)]
    fn from(val: u8) -> PlugContact {
        PlugContact::from_bits(val)
    }
}
impl From<PlugContact> for u8 {
    #[inline(always)]
    fn from(val: PlugContact) -> u8 {
        PlugContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdClrRxpwd1pt1 {
    #[doc = "Normal operation."]
    RXPWD1PT1_0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    RXPWD1PT1_1 = 0x01,
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
    RXPWDDIFF_0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    RXPWDDIFF_1 = 0x01,
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
    RXPWDENV_0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    RXPWDENV_1 = 0x01,
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
    RXPWDRX_0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    RXPWDRX_1 = 0x01,
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
    TXPWDFS_0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    TXPWDFS_1 = 0x01,
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
    TXPWDIBIAS_0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    TXPWDIBIAS_1 = 0x01,
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
    TXPWDV2I_0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    TXPWDV2I_1 = 0x01,
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
    RXPWD1PT1_0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    RXPWD1PT1_1 = 0x01,
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
    RXPWDDIFF_0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    RXPWDDIFF_1 = 0x01,
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
    RXPWDENV_0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    RXPWDENV_1 = 0x01,
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
    RXPWDRX_0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    RXPWDRX_1 = 0x01,
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
    RXPWD1PT1_0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    RXPWD1PT1_1 = 0x01,
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
    RXPWDDIFF_0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    RXPWDDIFF_1 = 0x01,
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
    RXPWDENV_0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    RXPWDENV_1 = 0x01,
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
    RXPWDRX_0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    RXPWDRX_1 = 0x01,
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
    TXPWDFS_0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    TXPWDFS_1 = 0x01,
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
    TXPWDIBIAS_0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    TXPWDIBIAS_1 = 0x01,
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
    TXPWDV2I_0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    TXPWDV2I_1 = 0x01,
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
    RXPWD1PT1_0 = 0x0,
    #[doc = "Power-down the USB full-speed differential receiver."]
    RXPWD1PT1_1 = 0x01,
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
    RXPWDDIFF_0 = 0x0,
    #[doc = "Power-down the USB high-speed differential receive"]
    RXPWDDIFF_1 = 0x01,
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
    RXPWDENV_0 = 0x0,
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    RXPWDENV_1 = 0x01,
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
    RXPWDRX_0 = 0x0,
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    RXPWDRX_1 = 0x01,
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
    TXPWDFS_0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    TXPWDFS_1 = 0x01,
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
    TXPWDIBIAS_0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    TXPWDIBIAS_1 = 0x01,
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
    TXPWDV2I_0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    TXPWDV2I_1 = 0x01,
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
    TXPWDFS_0 = 0x0,
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    TXPWDFS_1 = 0x01,
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
    TXPWDIBIAS_0 = 0x0,
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    TXPWDIBIAS_1 = 0x01,
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
    TXPWDV2I_0 = 0x0,
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    TXPWDV2I_1 = 0x01,
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
    DISCONADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    DISCONADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    DISCONADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    DISCONADJ_3 = 0x03,
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
    ENVADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    ENVADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    ENVADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    ENVADJ_3 = 0x03,
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
    RXDBYPASS_0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    RXDBYPASS_1 = 0x01,
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
    DISCONADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    DISCONADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    DISCONADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    DISCONADJ_3 = 0x03,
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
    ENVADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    ENVADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    ENVADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    ENVADJ_3 = 0x03,
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
    RXDBYPASS_0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    RXDBYPASS_1 = 0x01,
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
    DISCONADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    DISCONADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    DISCONADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    DISCONADJ_3 = 0x03,
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
    ENVADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    ENVADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    ENVADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    ENVADJ_3 = 0x03,
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
    RXDBYPASS_0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    RXDBYPASS_1 = 0x01,
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
    DISCONADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.55000 V"]
    DISCONADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.58125 V"]
    DISCONADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.60000 V"]
    DISCONADJ_3 = 0x03,
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
    ENVADJ_0 = 0x0,
    #[doc = "Trip-Level Voltage is 0.1125 V"]
    ENVADJ_1 = 0x01,
    #[doc = "Trip-Level Voltage is 0.1250 V"]
    ENVADJ_2 = 0x02,
    #[doc = "Trip-Level Voltage is 0.0875 V"]
    ENVADJ_3 = 0x03,
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
    RXDBYPASS_0 = 0x0,
    #[doc = "Use the output of the USB_DP single-ended receiver in place of the full-speed differential receiver"]
    RXDBYPASS_1 = 0x01,
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
pub enum SecdetDcp {
    #[doc = "Charging Downstream Port (CDP) has been detected"]
    SECDET_DCP_0 = 0x0,
    #[doc = "Downstream Charging Port (DCP) has been detected"]
    SECDET_DCP_1 = 0x01,
}
impl SecdetDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecdetDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecdetDcp {
    #[inline(always)]
    fn from(val: u8) -> SecdetDcp {
        SecdetDcp::from_bits(val)
    }
}
impl From<SecdetDcp> for u8 {
    #[inline(always)]
    fn from(val: SecdetDcp) -> u8 {
        SecdetDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sessend {
    #[doc = "The VBUS voltage is above the Session Valid threshold"]
    SESSEND_0 = 0x0,
    #[doc = "The VBUS voltage is below the Session Valid threshold"]
    SESSEND_1 = 0x01,
}
impl Sessend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sessend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sessend {
    #[inline(always)]
    fn from(val: u8) -> Sessend {
        Sessend::from_bits(val)
    }
}
impl From<Sessend> for u8 {
    #[inline(always)]
    fn from(val: Sessend) -> u8 {
        Sessend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxClrDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    D_CAL_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    D_CAL_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    D_CAL_15 = 0x0f,
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
    D_CAL_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    D_CAL_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    D_CAL_15 = 0x0f,
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
    D_CAL_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    D_CAL_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    D_CAL_15 = 0x0f,
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
    D_CAL_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal"]
    D_CAL_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    D_CAL_15 = 0x0f,
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
pub enum Usb1ChrgDetectBgrIbias {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    BGR_IBIAS_0 = 0x0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    BGR_IBIAS_1 = 0x01,
}
impl Usb1ChrgDetectBgrIbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectBgrIbias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectBgrIbias {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectBgrIbias {
        Usb1ChrgDetectBgrIbias::from_bits(val)
    }
}
impl From<Usb1ChrgDetectBgrIbias> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectBgrIbias) -> u8 {
        Usb1ChrgDetectBgrIbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectClrBgrIbias {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    BGR_IBIAS_0 = 0x0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    BGR_IBIAS_1 = 0x01,
}
impl Usb1ChrgDetectClrBgrIbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectClrBgrIbias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectClrBgrIbias {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectClrBgrIbias {
        Usb1ChrgDetectClrBgrIbias::from_bits(val)
    }
}
impl From<Usb1ChrgDetectClrBgrIbias> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectClrBgrIbias) -> u8 {
        Usb1ChrgDetectClrBgrIbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectSetBgrIbias {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    BGR_IBIAS_0 = 0x0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    BGR_IBIAS_1 = 0x01,
}
impl Usb1ChrgDetectSetBgrIbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectSetBgrIbias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectSetBgrIbias {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectSetBgrIbias {
        Usb1ChrgDetectSetBgrIbias::from_bits(val)
    }
}
impl From<Usb1ChrgDetectSetBgrIbias> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectSetBgrIbias) -> u8 {
        Usb1ChrgDetectSetBgrIbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectTogBgrIbias {
    #[doc = "Bias current is derived from the USB PHY internal current generator."]
    BGR_IBIAS_0 = 0x0,
    #[doc = "Bias current is derived from the reference generator of the bandgap."]
    BGR_IBIAS_1 = 0x01,
}
impl Usb1ChrgDetectTogBgrIbias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectTogBgrIbias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectTogBgrIbias {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectTogBgrIbias {
        Usb1ChrgDetectTogBgrIbias::from_bits(val)
    }
}
impl From<Usb1ChrgDetectTogBgrIbias> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectTogBgrIbias) -> u8 {
        Usb1ChrgDetectTogBgrIbias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrDischargeVbus {
    #[doc = "VBUS discharge resistor is disabled (Default)"]
    DISCHARGE_VBUS_0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    DISCHARGE_VBUS_1 = 0x01,
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
pub enum Usb1VbusDetectClrEnChargerResistor {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_0 = 0x0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_1 = 0x01,
}
impl Usb1VbusDetectClrEnChargerResistor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrEnChargerResistor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrEnChargerResistor {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrEnChargerResistor {
        Usb1VbusDetectClrEnChargerResistor::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrEnChargerResistor> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrEnChargerResistor) -> u8 {
        Usb1VbusDetectClrEnChargerResistor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    PWRUP_CMPS_0 = 0x0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    PWRUP_CMPS_1 = 0x01,
}
impl Usb1VbusDetectClrPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrPwrupCmps {
        unsafe { core::mem::transmute(val & 0x01) }
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
    VBUS_OVERRIDE_EN_0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VBUS_OVERRIDE_EN_1 = 0x01,
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
    VBUS_SOURCE_SEL_0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_2 = 0x02,
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
    VBUSVALID_SEL_0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VBUSVALID_SEL_1 = 0x01,
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
    VBUSVALID_THRESH_0 = 0x0,
    #[doc = "4.1V"]
    VBUSVALID_THRESH_1 = 0x01,
    #[doc = "4.2V"]
    VBUSVALID_THRESH_2 = 0x02,
    #[doc = "4.3V"]
    VBUSVALID_THRESH_3 = 0x03,
    #[doc = "4.4V(Default)"]
    VBUSVALID_THRESH_4 = 0x04,
    #[doc = "4.5V"]
    VBUSVALID_THRESH_5 = 0x05,
    #[doc = "4.6V"]
    VBUSVALID_THRESH_6 = 0x06,
    #[doc = "4.7V"]
    VBUSVALID_THRESH_7 = 0x07,
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
    VBUSVALID_TO_SESSVALID_0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VBUSVALID_TO_SESSVALID_1 = 0x01,
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
    DISCHARGE_VBUS_0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    DISCHARGE_VBUS_1 = 0x01,
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
pub enum Usb1VbusDetectEnChargerResistor {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_0 = 0x0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_1 = 0x01,
}
impl Usb1VbusDetectEnChargerResistor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectEnChargerResistor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectEnChargerResistor {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectEnChargerResistor {
        Usb1VbusDetectEnChargerResistor::from_bits(val)
    }
}
impl From<Usb1VbusDetectEnChargerResistor> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectEnChargerResistor) -> u8 {
        Usb1VbusDetectEnChargerResistor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    PWRUP_CMPS_0 = 0x0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    PWRUP_CMPS_1 = 0x01,
}
impl Usb1VbusDetectPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectPwrupCmps {
        unsafe { core::mem::transmute(val & 0x01) }
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
    DISCHARGE_VBUS_0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    DISCHARGE_VBUS_1 = 0x01,
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
pub enum Usb1VbusDetectSetEnChargerResistor {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_0 = 0x0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_1 = 0x01,
}
impl Usb1VbusDetectSetEnChargerResistor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetEnChargerResistor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetEnChargerResistor {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetEnChargerResistor {
        Usb1VbusDetectSetEnChargerResistor::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetEnChargerResistor> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetEnChargerResistor) -> u8 {
        Usb1VbusDetectSetEnChargerResistor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    PWRUP_CMPS_0 = 0x0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    PWRUP_CMPS_1 = 0x01,
}
impl Usb1VbusDetectSetPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetPwrupCmps {
        unsafe { core::mem::transmute(val & 0x01) }
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
    VBUS_OVERRIDE_EN_0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VBUS_OVERRIDE_EN_1 = 0x01,
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
    VBUS_SOURCE_SEL_0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_2 = 0x02,
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
    VBUSVALID_SEL_0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VBUSVALID_SEL_1 = 0x01,
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
    VBUSVALID_THRESH_0 = 0x0,
    #[doc = "4.1V"]
    VBUSVALID_THRESH_1 = 0x01,
    #[doc = "4.2V"]
    VBUSVALID_THRESH_2 = 0x02,
    #[doc = "4.3V"]
    VBUSVALID_THRESH_3 = 0x03,
    #[doc = "4.4V(Default)"]
    VBUSVALID_THRESH_4 = 0x04,
    #[doc = "4.5V"]
    VBUSVALID_THRESH_5 = 0x05,
    #[doc = "4.6V"]
    VBUSVALID_THRESH_6 = 0x06,
    #[doc = "4.7V"]
    VBUSVALID_THRESH_7 = 0x07,
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
    VBUSVALID_TO_SESSVALID_0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VBUSVALID_TO_SESSVALID_1 = 0x01,
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
    DISCHARGE_VBUS_0 = 0x0,
    #[doc = "VBUS discharge resistor is enabled"]
    DISCHARGE_VBUS_1 = 0x01,
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
pub enum Usb1VbusDetectTogEnChargerResistor {
    #[doc = "Disable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_0 = 0x0,
    #[doc = "Enable resistive charger detection resistors on USB_DP and USB_DP"]
    EN_CHARGER_RESISTOR_1 = 0x01,
}
impl Usb1VbusDetectTogEnChargerResistor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogEnChargerResistor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogEnChargerResistor {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogEnChargerResistor {
        Usb1VbusDetectTogEnChargerResistor::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogEnChargerResistor> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogEnChargerResistor) -> u8 {
        Usb1VbusDetectTogEnChargerResistor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogPwrupCmps {
    #[doc = "Powers down the VBUS_VALID comparator"]
    PWRUP_CMPS_0 = 0x0,
    #[doc = "Enables the VBUS_VALID comparator (default)"]
    PWRUP_CMPS_1 = 0x01,
}
impl Usb1VbusDetectTogPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogPwrupCmps {
        unsafe { core::mem::transmute(val & 0x01) }
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
    VBUS_OVERRIDE_EN_0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VBUS_OVERRIDE_EN_1 = 0x01,
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
    VBUS_SOURCE_SEL_0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_2 = 0x02,
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
    VBUSVALID_SEL_0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VBUSVALID_SEL_1 = 0x01,
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
    VBUSVALID_THRESH_0 = 0x0,
    #[doc = "4.1V"]
    VBUSVALID_THRESH_1 = 0x01,
    #[doc = "4.2V"]
    VBUSVALID_THRESH_2 = 0x02,
    #[doc = "4.3V"]
    VBUSVALID_THRESH_3 = 0x03,
    #[doc = "4.4V(Default)"]
    VBUSVALID_THRESH_4 = 0x04,
    #[doc = "4.5V"]
    VBUSVALID_THRESH_5 = 0x05,
    #[doc = "4.6V"]
    VBUSVALID_THRESH_6 = 0x06,
    #[doc = "4.7V"]
    VBUSVALID_THRESH_7 = 0x07,
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
    VBUSVALID_TO_SESSVALID_0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VBUSVALID_TO_SESSVALID_1 = 0x01,
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
    VBUS_OVERRIDE_EN_0 = 0x0,
    #[doc = "Use the override values for VBUS_VALID, AVALID, BVALID, and SESSEND"]
    VBUS_OVERRIDE_EN_1 = 0x01,
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
    VBUS_SOURCE_SEL_0 = 0x0,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_1 = 0x01,
    #[doc = "Use the Session Valid comparator results for signal reported to the USB controller"]
    VBUS_SOURCE_SEL_2 = 0x02,
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
    VBUSVALID_SEL_0 = 0x0,
    #[doc = "Use the VBUS_VALID_3V detector results for signal reported to the USB controller"]
    VBUSVALID_SEL_1 = 0x01,
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
    VBUSVALID_THRESH_0 = 0x0,
    #[doc = "4.1V"]
    VBUSVALID_THRESH_1 = 0x01,
    #[doc = "4.2V"]
    VBUSVALID_THRESH_2 = 0x02,
    #[doc = "4.3V"]
    VBUSVALID_THRESH_3 = 0x03,
    #[doc = "4.4V(Default)"]
    VBUSVALID_THRESH_4 = 0x04,
    #[doc = "4.5V"]
    VBUSVALID_THRESH_5 = 0x05,
    #[doc = "4.6V"]
    VBUSVALID_THRESH_6 = 0x06,
    #[doc = "4.7V"]
    VBUSVALID_THRESH_7 = 0x07,
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
    VBUSVALID_TO_SESSVALID_0 = 0x0,
    #[doc = "Use the Session End comparator for VBUS_VALID results. The Session End threshold is >0.8V and <4.0V."]
    VBUSVALID_TO_SESSVALID_1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid {
    #[doc = "VBUS is below the comparator threshold"]
    VBUS_VALID_0 = 0x0,
    #[doc = "VBUS is above the comparator threshold"]
    VBUS_VALID_1 = 0x01,
}
impl VbusValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid {
    #[inline(always)]
    fn from(val: u8) -> VbusValid {
        VbusValid::from_bits(val)
    }
}
impl From<VbusValid> for u8 {
    #[inline(always)]
    fn from(val: VbusValid) -> u8 {
        VbusValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid3v {
    #[doc = "VBUS voltage is below VBUS_VALID_3V threshold"]
    VBUS_VALID_3V_0 = 0x0,
    #[doc = "VBUS voltage is above VBUS_VALID_3V threshold"]
    VBUS_VALID_3V_1 = 0x01,
}
impl VbusValid3v {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid3v {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid3v {
    #[inline(always)]
    fn from(val: u8) -> VbusValid3v {
        VbusValid3v::from_bits(val)
    }
}
impl From<VbusValid3v> for u8 {
    #[inline(always)]
    fn from(val: VbusValid3v) -> u8 {
        VbusValid3v::to_bits(val)
    }
}
