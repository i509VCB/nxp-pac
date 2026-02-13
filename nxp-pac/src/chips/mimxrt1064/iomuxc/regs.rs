#[doc = "ANATOP_USB_OTG1_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnatopUsbOtg1IdSelectInput(pub u32);
impl AnatopUsbOtg1IdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::AnatopUsbOtg1IdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AnatopUsbOtg1IdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::AnatopUsbOtg1IdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for AnatopUsbOtg1IdSelectInput {
    #[inline(always)]
    fn default() -> AnatopUsbOtg1IdSelectInput {
        AnatopUsbOtg1IdSelectInput(0)
    }
}
impl core::fmt::Debug for AnatopUsbOtg1IdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnatopUsbOtg1IdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnatopUsbOtg1IdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnatopUsbOtg1IdSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ANATOP_USB_OTG2_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnatopUsbOtg2IdSelectInput(pub u32);
impl AnatopUsbOtg2IdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::AnatopUsbOtg2IdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AnatopUsbOtg2IdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::AnatopUsbOtg2IdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for AnatopUsbOtg2IdSelectInput {
    #[inline(always)]
    fn default() -> AnatopUsbOtg2IdSelectInput {
        AnatopUsbOtg2IdSelectInput(0)
    }
}
impl core::fmt::Debug for AnatopUsbOtg2IdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnatopUsbOtg2IdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnatopUsbOtg2IdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnatopUsbOtg2IdSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "CANFD_IPP_IND_CANRX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanfdIppIndCanrxSelectInput(pub u32);
impl CanfdIppIndCanrxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CanfdIppIndCanrxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CanfdIppIndCanrxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CanfdIppIndCanrxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for CanfdIppIndCanrxSelectInput {
    #[inline(always)]
    fn default() -> CanfdIppIndCanrxSelectInput {
        CanfdIppIndCanrxSelectInput(0)
    }
}
impl core::fmt::Debug for CanfdIppIndCanrxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CanfdIppIndCanrxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CanfdIppIndCanrxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CanfdIppIndCanrxSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "CCM_PMIC_READY_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcmPmicReadySelectInput(pub u32);
impl CcmPmicReadySelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CcmPmicReadySelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CcmPmicReadySelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CcmPmicReadySelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for CcmPmicReadySelectInput {
    #[inline(always)]
    fn default() -> CcmPmicReadySelectInput {
        CcmPmicReadySelectInput(0)
    }
}
impl core::fmt::Debug for CcmPmicReadySelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcmPmicReadySelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CcmPmicReadySelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CcmPmicReadySelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA02_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData02SelectInput(pub u32);
impl CsiData02SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData02SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData02SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData02SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData02SelectInput {
    #[inline(always)]
    fn default() -> CsiData02SelectInput {
        CsiData02SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData02SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData02SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData02SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData02SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA03_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData03SelectInput(pub u32);
impl CsiData03SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData03SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData03SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData03SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData03SelectInput {
    #[inline(always)]
    fn default() -> CsiData03SelectInput {
        CsiData03SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData03SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData03SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData03SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData03SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA04_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData04SelectInput(pub u32);
impl CsiData04SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData04SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData04SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData04SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData04SelectInput {
    #[inline(always)]
    fn default() -> CsiData04SelectInput {
        CsiData04SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData04SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData04SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData04SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData04SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA05_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData05SelectInput(pub u32);
impl CsiData05SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData05SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData05SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData05SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData05SelectInput {
    #[inline(always)]
    fn default() -> CsiData05SelectInput {
        CsiData05SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData05SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData05SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData05SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData05SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA06_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData06SelectInput(pub u32);
impl CsiData06SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData06SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData06SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData06SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData06SelectInput {
    #[inline(always)]
    fn default() -> CsiData06SelectInput {
        CsiData06SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData06SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData06SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData06SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData06SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA07_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData07SelectInput(pub u32);
impl CsiData07SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData07SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData07SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData07SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData07SelectInput {
    #[inline(always)]
    fn default() -> CsiData07SelectInput {
        CsiData07SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData07SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData07SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData07SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData07SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA08_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData08SelectInput(pub u32);
impl CsiData08SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData08SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData08SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData08SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData08SelectInput {
    #[inline(always)]
    fn default() -> CsiData08SelectInput {
        CsiData08SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData08SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData08SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData08SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData08SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_DATA09_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiData09SelectInput(pub u32);
impl CsiData09SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiData09SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiData09SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiData09SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiData09SelectInput {
    #[inline(always)]
    fn default() -> CsiData09SelectInput {
        CsiData09SelectInput(0)
    }
}
impl core::fmt::Debug for CsiData09SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiData09SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiData09SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiData09SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_HSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiHsyncSelectInput(pub u32);
impl CsiHsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiHsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CsiHsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiHsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for CsiHsyncSelectInput {
    #[inline(always)]
    fn default() -> CsiHsyncSelectInput {
        CsiHsyncSelectInput(0)
    }
}
impl core::fmt::Debug for CsiHsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiHsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiHsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiHsyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_PIXCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiPixclkSelectInput(pub u32);
impl CsiPixclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiPixclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CsiPixclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiPixclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CsiPixclkSelectInput {
    #[inline(always)]
    fn default() -> CsiPixclkSelectInput {
        CsiPixclkSelectInput(0)
    }
}
impl core::fmt::Debug for CsiPixclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiPixclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiPixclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiPixclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "CSI_VSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsiVsyncSelectInput(pub u32);
impl CsiVsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::CsiVsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CsiVsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::CsiVsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for CsiVsyncSelectInput {
    #[inline(always)]
    fn default() -> CsiVsyncSelectInput {
        CsiVsyncSelectInput(0)
    }
}
impl core::fmt::Debug for CsiVsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsiVsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsiVsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CsiVsyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SW_PAD_CTL_PAD_GPIO_AD_B0_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::Speed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Speed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::Speed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::Pus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::Pus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
impl core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl {{ sre: {=bool:?}, dse: {:?}, speed: {:?}, ode: {=bool:?}, pke: {=bool:?}, pue: {=bool:?}, pus: {:?}, hys: {=bool:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "ENET0_RXDATA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0RxdataSelectInput(pub u32);
impl Enet0RxdataSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet0RxdataSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enet0RxdataSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet0RxdataSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enet0RxdataSelectInput {
    #[inline(always)]
    fn default() -> Enet0RxdataSelectInput {
        Enet0RxdataSelectInput(0)
    }
}
impl core::fmt::Debug for Enet0RxdataSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet0RxdataSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet0RxdataSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enet0RxdataSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET0_TIMER_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0TimerSelectInput(pub u32);
impl Enet0TimerSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet0TimerSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet0TimerSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet0TimerSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet0TimerSelectInput {
    #[inline(always)]
    fn default() -> Enet0TimerSelectInput {
        Enet0TimerSelectInput(0)
    }
}
impl core::fmt::Debug for Enet0TimerSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet0TimerSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet0TimerSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enet0TimerSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET1_RXDATA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet1RxdataSelectInput(pub u32);
impl Enet1RxdataSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet1RxdataSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enet1RxdataSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet1RxdataSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enet1RxdataSelectInput {
    #[inline(always)]
    fn default() -> Enet1RxdataSelectInput {
        Enet1RxdataSelectInput(0)
    }
}
impl core::fmt::Debug for Enet1RxdataSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet1RxdataSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet1RxdataSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enet1RxdataSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET2_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IpgClkRmiiSelectInput(pub u32);
impl Enet2IpgClkRmiiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IpgClkRmiiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IpgClkRmiiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IpgClkRmiiSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IpgClkRmiiSelectInput {
    #[inline(always)]
    fn default() -> Enet2IpgClkRmiiSelectInput {
        Enet2IpgClkRmiiSelectInput(0)
    }
}
impl core::fmt::Debug for Enet2IpgClkRmiiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IpgClkRmiiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IpgClkRmiiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IpgClkRmiiSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_MDIO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0MdioSelectInput(pub u32);
impl Enet2IppIndMac0MdioSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0MdioSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enet2IppIndMac0MdioSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0MdioSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enet2IppIndMac0MdioSelectInput {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0MdioSelectInput {
        Enet2IppIndMac0MdioSelectInput(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0MdioSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0MdioSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0MdioSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0MdioSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0RxdataSelectInput0(pub u32);
impl Enet2IppIndMac0RxdataSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0RxdataSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IppIndMac0RxdataSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0RxdataSelectInput0Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IppIndMac0RxdataSelectInput0 {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0RxdataSelectInput0 {
        Enet2IppIndMac0RxdataSelectInput0(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0RxdataSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0RxdataSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0RxdataSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0RxdataSelectInput0 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_RXDATA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0RxdataSelectInput1(pub u32);
impl Enet2IppIndMac0RxdataSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0RxdataSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IppIndMac0RxdataSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0RxdataSelectInput1Daisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IppIndMac0RxdataSelectInput1 {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0RxdataSelectInput1 {
        Enet2IppIndMac0RxdataSelectInput1(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0RxdataSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0RxdataSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0RxdataSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0RxdataSelectInput1 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_RXEN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0RxenSelectInput(pub u32);
impl Enet2IppIndMac0RxenSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0RxenSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IppIndMac0RxenSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0RxenSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IppIndMac0RxenSelectInput {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0RxenSelectInput {
        Enet2IppIndMac0RxenSelectInput(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0RxenSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0RxenSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0RxenSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0RxenSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_RXERR_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0RxerrSelectInput(pub u32);
impl Enet2IppIndMac0RxerrSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0RxerrSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IppIndMac0RxerrSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0RxerrSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IppIndMac0RxerrSelectInput {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0RxerrSelectInput {
        Enet2IppIndMac0RxerrSelectInput(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0RxerrSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0RxerrSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0RxerrSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0RxerrSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_TIMER_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0TimerSelectInput0(pub u32);
impl Enet2IppIndMac0TimerSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0TimerSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enet2IppIndMac0TimerSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0TimerSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Enet2IppIndMac0TimerSelectInput0 {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0TimerSelectInput0 {
        Enet2IppIndMac0TimerSelectInput0(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0TimerSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0TimerSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0TimerSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0TimerSelectInput0 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET2_IPP_IND_MAC0_TXCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet2IppIndMac0TxclkSelectInput(pub u32);
impl Enet2IppIndMac0TxclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Enet2IppIndMac0TxclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enet2IppIndMac0TxclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Enet2IppIndMac0TxclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Enet2IppIndMac0TxclkSelectInput {
    #[inline(always)]
    fn default() -> Enet2IppIndMac0TxclkSelectInput {
        Enet2IppIndMac0TxclkSelectInput(0)
    }
}
impl core::fmt::Debug for Enet2IppIndMac0TxclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enet2IppIndMac0TxclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enet2IppIndMac0TxclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enet2IppIndMac0TxclkSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET_IPG_CLK_RMII_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetIpgClkRmiiSelectInput(pub u32);
impl EnetIpgClkRmiiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EnetIpgClkRmiiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EnetIpgClkRmiiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EnetIpgClkRmiiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetIpgClkRmiiSelectInput {
    #[inline(always)]
    fn default() -> EnetIpgClkRmiiSelectInput {
        EnetIpgClkRmiiSelectInput(0)
    }
}
impl core::fmt::Debug for EnetIpgClkRmiiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetIpgClkRmiiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetIpgClkRmiiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetIpgClkRmiiSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "ENET_MDIO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetMdioSelectInput(pub u32);
impl EnetMdioSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EnetMdioSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::EnetMdioSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EnetMdioSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for EnetMdioSelectInput {
    #[inline(always)]
    fn default() -> EnetMdioSelectInput {
        EnetMdioSelectInput(0)
    }
}
impl core::fmt::Debug for EnetMdioSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetMdioSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetMdioSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetMdioSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET_RXEN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetRxenSelectInput(pub u32);
impl EnetRxenSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EnetRxenSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EnetRxenSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EnetRxenSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetRxenSelectInput {
    #[inline(always)]
    fn default() -> EnetRxenSelectInput {
        EnetRxenSelectInput(0)
    }
}
impl core::fmt::Debug for EnetRxenSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetRxenSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetRxenSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetRxenSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET_RXERR_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetRxerrSelectInput(pub u32);
impl EnetRxerrSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EnetRxerrSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EnetRxerrSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EnetRxerrSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetRxerrSelectInput {
    #[inline(always)]
    fn default() -> EnetRxerrSelectInput {
        EnetRxerrSelectInput(0)
    }
}
impl core::fmt::Debug for EnetRxerrSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetRxerrSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetRxerrSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetRxerrSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "ENET_TXCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetTxclkSelectInput(pub u32);
impl EnetTxclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::EnetTxclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EnetTxclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::EnetTxclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetTxclkSelectInput {
    #[inline(always)]
    fn default() -> EnetTxclkSelectInput {
        EnetTxclkSelectInput(0)
    }
}
impl core::fmt::Debug for EnetTxclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetTxclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetTxclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetTxclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXCAN1_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan1RxSelectInput(pub u32);
impl Flexcan1RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexcan1RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexcan1RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexcan1RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexcan1RxSelectInput {
    #[inline(always)]
    fn default() -> Flexcan1RxSelectInput {
        Flexcan1RxSelectInput(0)
    }
}
impl core::fmt::Debug for Flexcan1RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan1RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan1RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan1RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXCAN2_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan2RxSelectInput(pub u32);
impl Flexcan2RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexcan2RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexcan2RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexcan2RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexcan2RxSelectInput {
    #[inline(always)]
    fn default() -> Flexcan2RxSelectInput {
        Flexcan2RxSelectInput(0)
    }
}
impl core::fmt::Debug for Flexcan2RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan2RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan2RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan2RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXPWM1_PWMA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwma0SelectInput(pub u32);
impl Flexpwm1Pwma0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwma0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwma0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwma0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwma0SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwma0SelectInput {
        Flexpwm1Pwma0SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwma0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwma0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwma0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwma0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwma1SelectInput(pub u32);
impl Flexpwm1Pwma1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwma1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwma1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwma1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwma1SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwma1SelectInput {
        Flexpwm1Pwma1SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwma1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwma1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwma1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwma1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwma2SelectInput(pub u32);
impl Flexpwm1Pwma2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwma2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwma2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwma2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwma2SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwma2SelectInput {
        Flexpwm1Pwma2SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwma2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwma2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwma2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwma2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwma3SelectInput(pub u32);
impl Flexpwm1Pwma3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwma3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flexpwm1Pwma3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwma3SelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexpwm1Pwma3SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwma3SelectInput {
        Flexpwm1Pwma3SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwma3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwma3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwma3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwma3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwmb0SelectInput(pub u32);
impl Flexpwm1Pwmb0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwmb0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwmb0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwmb0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwmb0SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwmb0SelectInput {
        Flexpwm1Pwmb0SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwmb0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwmb0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwmb0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwmb0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwmb1SelectInput(pub u32);
impl Flexpwm1Pwmb1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwmb1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwmb1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwmb1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwmb1SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwmb1SelectInput {
        Flexpwm1Pwmb1SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwmb1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwmb1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwmb1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwmb1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwmb2SelectInput(pub u32);
impl Flexpwm1Pwmb2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwmb2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1Pwmb2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwmb2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1Pwmb2SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwmb2SelectInput {
        Flexpwm1Pwmb2SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwmb2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwmb2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwmb2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwmb2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1Pwmb3SelectInput(pub u32);
impl Flexpwm1Pwmb3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1Pwmb3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flexpwm1Pwmb3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1Pwmb3SelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexpwm1Pwmb3SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm1Pwmb3SelectInput {
        Flexpwm1Pwmb3SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm1Pwmb3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1Pwmb3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1Pwmb3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1Pwmb3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwma0SelectInput(pub u32);
impl Flexpwm2Pwma0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwma0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwma0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwma0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwma0SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwma0SelectInput {
        Flexpwm2Pwma0SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwma0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwma0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwma0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwma0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwma1SelectInput(pub u32);
impl Flexpwm2Pwma1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwma1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwma1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwma1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwma1SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwma1SelectInput {
        Flexpwm2Pwma1SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwma1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwma1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwma1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwma1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwma2SelectInput(pub u32);
impl Flexpwm2Pwma2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwma2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwma2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwma2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwma2SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwma2SelectInput {
        Flexpwm2Pwma2SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwma2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwma2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwma2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwma2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwma3SelectInput(pub u32);
impl Flexpwm2Pwma3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwma3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Flexpwm2Pwma3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwma3SelectInputDaisy) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexpwm2Pwma3SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwma3SelectInput {
        Flexpwm2Pwma3SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwma3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwma3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwma3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwma3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMB0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwmb0SelectInput(pub u32);
impl Flexpwm2Pwmb0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwmb0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwmb0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwmb0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwmb0SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwmb0SelectInput {
        Flexpwm2Pwmb0SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwmb0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwmb0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwmb0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwmb0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMB1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwmb1SelectInput(pub u32);
impl Flexpwm2Pwmb1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwmb1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwmb1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwmb1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwmb1SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwmb1SelectInput {
        Flexpwm2Pwmb1SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwmb1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwmb1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwmb1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwmb1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMB2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwmb2SelectInput(pub u32);
impl Flexpwm2Pwmb2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwmb2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm2Pwmb2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwmb2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm2Pwmb2SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwmb2SelectInput {
        Flexpwm2Pwmb2SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwmb2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwmb2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwmb2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwmb2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM2_PWMB3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm2Pwmb3SelectInput(pub u32);
impl Flexpwm2Pwmb3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm2Pwmb3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexpwm2Pwmb3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm2Pwmb3SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Flexpwm2Pwmb3SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm2Pwmb3SelectInput {
        Flexpwm2Pwmb3SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm2Pwmb3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm2Pwmb3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm2Pwmb3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm2Pwmb3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM4_PWMA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm4Pwma0SelectInput(pub u32);
impl Flexpwm4Pwma0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm4Pwma0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm4Pwma0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm4Pwma0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm4Pwma0SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm4Pwma0SelectInput {
        Flexpwm4Pwma0SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm4Pwma0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm4Pwma0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm4Pwma0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm4Pwma0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM4_PWMA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm4Pwma1SelectInput(pub u32);
impl Flexpwm4Pwma1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm4Pwma1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm4Pwma1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm4Pwma1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm4Pwma1SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm4Pwma1SelectInput {
        Flexpwm4Pwma1SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm4Pwma1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm4Pwma1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm4Pwma1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm4Pwma1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM4_PWMA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm4Pwma2SelectInput(pub u32);
impl Flexpwm4Pwma2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm4Pwma2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm4Pwma2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm4Pwma2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm4Pwma2SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm4Pwma2SelectInput {
        Flexpwm4Pwma2SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm4Pwma2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm4Pwma2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm4Pwma2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm4Pwma2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM4_PWMA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm4Pwma3SelectInput(pub u32);
impl Flexpwm4Pwma3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm4Pwma3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm4Pwma3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm4Pwma3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm4Pwma3SelectInput {
    #[inline(always)]
    fn default() -> Flexpwm4Pwma3SelectInput {
        Flexpwm4Pwma3SelectInput(0)
    }
}
impl core::fmt::Debug for Flexpwm4Pwma3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm4Pwma3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm4Pwma3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm4Pwma3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIA_DATA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaData0SelectInput(pub u32);
impl FlexspiaData0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaData0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaData0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaData0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaData0SelectInput {
    #[inline(always)]
    fn default() -> FlexspiaData0SelectInput {
        FlexspiaData0SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaData0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaData0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaData0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspiaData0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIA_DATA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaData1SelectInput(pub u32);
impl FlexspiaData1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaData1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaData1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaData1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaData1SelectInput {
    #[inline(always)]
    fn default() -> FlexspiaData1SelectInput {
        FlexspiaData1SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaData1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaData1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaData1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspiaData1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIA_DATA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaData2SelectInput(pub u32);
impl FlexspiaData2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaData2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaData2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaData2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaData2SelectInput {
    #[inline(always)]
    fn default() -> FlexspiaData2SelectInput {
        FlexspiaData2SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaData2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaData2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaData2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspiaData2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIA_DATA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaData3SelectInput(pub u32);
impl FlexspiaData3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaData3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaData3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaData3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaData3SelectInput {
    #[inline(always)]
    fn default() -> FlexspiaData3SelectInput {
        FlexspiaData3SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaData3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaData3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaData3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspiaData3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIA_DQS_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaDqsSelectInput(pub u32);
impl FlexspiaDqsSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaDqsSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaDqsSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaDqsSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaDqsSelectInput {
    #[inline(always)]
    fn default() -> FlexspiaDqsSelectInput {
        FlexspiaDqsSelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaDqsSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaDqsSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaDqsSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexspiaDqsSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXSPIA_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiaSckSelectInput(pub u32);
impl FlexspiaSckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiaSckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiaSckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiaSckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiaSckSelectInput {
    #[inline(always)]
    fn default() -> FlexspiaSckSelectInput {
        FlexspiaSckSelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiaSckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiaSckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiaSckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexspiaSckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXSPIB_DATA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspibData0SelectInput(pub u32);
impl FlexspibData0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspibData0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspibData0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspibData0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspibData0SelectInput {
    #[inline(always)]
    fn default() -> FlexspibData0SelectInput {
        FlexspibData0SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspibData0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspibData0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspibData0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspibData0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIB_DATA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspibData1SelectInput(pub u32);
impl FlexspibData1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspibData1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspibData1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspibData1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspibData1SelectInput {
    #[inline(always)]
    fn default() -> FlexspibData1SelectInput {
        FlexspibData1SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspibData1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspibData1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspibData1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspibData1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIB_DATA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspibData2SelectInput(pub u32);
impl FlexspibData2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspibData2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspibData2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspibData2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspibData2SelectInput {
    #[inline(always)]
    fn default() -> FlexspibData2SelectInput {
        FlexspibData2SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspibData2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspibData2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspibData2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspibData2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPIB_DATA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspibData3SelectInput(pub u32);
impl FlexspibData3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspibData3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspibData3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspibData3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspibData3SelectInput {
    #[inline(always)]
    fn default() -> FlexspibData3SelectInput {
        FlexspibData3SelectInput(0)
    }
}
impl core::fmt::Debug for FlexspibData3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspibData3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspibData3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspibData3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT1_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt1IppIndCapin1SelectInput(pub u32);
impl Gpt1IppIndCapin1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt1IppIndCapin1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt1IppIndCapin1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt1IppIndCapin1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt1IppIndCapin1SelectInput {
    #[inline(always)]
    fn default() -> Gpt1IppIndCapin1SelectInput {
        Gpt1IppIndCapin1SelectInput(0)
    }
}
impl core::fmt::Debug for Gpt1IppIndCapin1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt1IppIndCapin1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt1IppIndCapin1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt1IppIndCapin1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT1_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt1IppIndCapin2SelectInput(pub u32);
impl Gpt1IppIndCapin2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt1IppIndCapin2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt1IppIndCapin2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt1IppIndCapin2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt1IppIndCapin2SelectInput {
    #[inline(always)]
    fn default() -> Gpt1IppIndCapin2SelectInput {
        Gpt1IppIndCapin2SelectInput(0)
    }
}
impl core::fmt::Debug for Gpt1IppIndCapin2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt1IppIndCapin2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt1IppIndCapin2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt1IppIndCapin2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT1_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt1IppIndClkinSelectInput(pub u32);
impl Gpt1IppIndClkinSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt1IppIndClkinSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt1IppIndClkinSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt1IppIndClkinSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt1IppIndClkinSelectInput {
    #[inline(always)]
    fn default() -> Gpt1IppIndClkinSelectInput {
        Gpt1IppIndClkinSelectInput(0)
    }
}
impl core::fmt::Debug for Gpt1IppIndClkinSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt1IppIndClkinSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt1IppIndClkinSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt1IppIndClkinSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT2_IPP_IND_CAPIN1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt2IppIndCapin1SelectInput(pub u32);
impl Gpt2IppIndCapin1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt2IppIndCapin1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt2IppIndCapin1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt2IppIndCapin1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt2IppIndCapin1SelectInput {
    #[inline(always)]
    fn default() -> Gpt2IppIndCapin1SelectInput {
        Gpt2IppIndCapin1SelectInput(0)
    }
}
impl core::fmt::Debug for Gpt2IppIndCapin1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt2IppIndCapin1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt2IppIndCapin1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt2IppIndCapin1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT2_IPP_IND_CAPIN2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt2IppIndCapin2SelectInput(pub u32);
impl Gpt2IppIndCapin2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt2IppIndCapin2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt2IppIndCapin2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt2IppIndCapin2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt2IppIndCapin2SelectInput {
    #[inline(always)]
    fn default() -> Gpt2IppIndCapin2SelectInput {
        Gpt2IppIndCapin2SelectInput(0)
    }
}
impl core::fmt::Debug for Gpt2IppIndCapin2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt2IppIndCapin2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt2IppIndCapin2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt2IppIndCapin2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "GPT2_IPP_IND_CLKIN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt2IppIndClkinSelectInput(pub u32);
impl Gpt2IppIndClkinSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Gpt2IppIndClkinSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gpt2IppIndClkinSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Gpt2IppIndClkinSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Gpt2IppIndClkinSelectInput {
    #[inline(always)]
    fn default() -> Gpt2IppIndClkinSelectInput {
        Gpt2IppIndClkinSelectInput(0)
    }
}
impl core::fmt::Debug for Gpt2IppIndClkinSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpt2IppIndClkinSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpt2IppIndClkinSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpt2IppIndClkinSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1SclSelectInput(pub u32);
impl Lpi2c1SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c1SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c1SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1SclSelectInput {
        Lpi2c1SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c1SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1SdaSelectInput(pub u32);
impl Lpi2c1SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c1SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c1SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1SdaSelectInput {
        Lpi2c1SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c1SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2SclSelectInput(pub u32);
impl Lpi2c2SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c2SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c2SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2SclSelectInput {
        Lpi2c2SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c2SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c2SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2SdaSelectInput(pub u32);
impl Lpi2c2SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c2SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c2SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2SdaSelectInput {
        Lpi2c2SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c2SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c2SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C3_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c3SclSelectInput(pub u32);
impl Lpi2c3SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c3SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c3SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c3SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c3SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c3SclSelectInput {
        Lpi2c3SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c3SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c3SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c3SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c3SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C3_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c3SdaSelectInput(pub u32);
impl Lpi2c3SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c3SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c3SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c3SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c3SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c3SdaSelectInput {
        Lpi2c3SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c3SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c3SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c3SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c3SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C4_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c4SclSelectInput(pub u32);
impl Lpi2c4SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c4SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c4SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c4SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c4SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c4SclSelectInput {
        Lpi2c4SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c4SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c4SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c4SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c4SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C4_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c4SdaSelectInput(pub u32);
impl Lpi2c4SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c4SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c4SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c4SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c4SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c4SdaSelectInput {
        Lpi2c4SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c4SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c4SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c4SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c4SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_PCS0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1Pcs0SelectInput(pub u32);
impl Lpspi1Pcs0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1Pcs0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1Pcs0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1Pcs0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1Pcs0SelectInput {
    #[inline(always)]
    fn default() -> Lpspi1Pcs0SelectInput {
        Lpspi1Pcs0SelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1Pcs0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1Pcs0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1Pcs0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1Pcs0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SckSelectInput(pub u32);
impl Lpspi1SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SckSelectInput {
        Lpspi1SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SdiSelectInput(pub u32);
impl Lpspi1SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SdiSelectInput {
        Lpspi1SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SdoSelectInput(pub u32);
impl Lpspi1SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SdoSelectInput {
        Lpspi1SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_PCS0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2Pcs0SelectInput(pub u32);
impl Lpspi2Pcs0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2Pcs0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2Pcs0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2Pcs0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2Pcs0SelectInput {
    #[inline(always)]
    fn default() -> Lpspi2Pcs0SelectInput {
        Lpspi2Pcs0SelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2Pcs0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2Pcs0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2Pcs0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2Pcs0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SckSelectInput(pub u32);
impl Lpspi2SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SckSelectInput {
        Lpspi2SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SdiSelectInput(pub u32);
impl Lpspi2SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SdiSelectInput {
        Lpspi2SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SdoSelectInput(pub u32);
impl Lpspi2SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SdoSelectInput {
        Lpspi2SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI3_PCS0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3Pcs0SelectInput(pub u32);
impl Lpspi3Pcs0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3Pcs0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3Pcs0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3Pcs0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3Pcs0SelectInput {
    #[inline(always)]
    fn default() -> Lpspi3Pcs0SelectInput {
        Lpspi3Pcs0SelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi3Pcs0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3Pcs0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3Pcs0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi3Pcs0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI3_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3SckSelectInput(pub u32);
impl Lpspi3SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3SckSelectInput {
        Lpspi3SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi3SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi3SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI3_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3SdiSelectInput(pub u32);
impl Lpspi3SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3SdiSelectInput {
        Lpspi3SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi3SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi3SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI3_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi3SdoSelectInput(pub u32);
impl Lpspi3SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi3SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi3SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi3SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi3SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi3SdoSelectInput {
        Lpspi3SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi3SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi3SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi3SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi3SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI4_PCS0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4Pcs0SelectInput(pub u32);
impl Lpspi4Pcs0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4Pcs0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi4Pcs0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4Pcs0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi4Pcs0SelectInput {
    #[inline(always)]
    fn default() -> Lpspi4Pcs0SelectInput {
        Lpspi4Pcs0SelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi4Pcs0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4Pcs0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4Pcs0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi4Pcs0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI4_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4SckSelectInput(pub u32);
impl Lpspi4SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi4SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi4SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4SckSelectInput {
        Lpspi4SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi4SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi4SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI4_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4SdiSelectInput(pub u32);
impl Lpspi4SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi4SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi4SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4SdiSelectInput {
        Lpspi4SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi4SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi4SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI4_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi4SdoSelectInput(pub u32);
impl Lpspi4SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi4SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi4SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi4SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi4SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi4SdoSelectInput {
        Lpspi4SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi4SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi4SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi4SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi4SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART2_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2RxSelectInput(pub u32);
impl Lpuart2RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2RxSelectInput {
        Lpuart2RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart2RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart2RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART2_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2TxSelectInput(pub u32);
impl Lpuart2TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2TxSelectInput {
        Lpuart2TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart2TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart2TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART3_CTS_B_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3CtsBSelectInput(pub u32);
impl Lpuart3CtsBSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3CtsBSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart3CtsBSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3CtsBSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart3CtsBSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3CtsBSelectInput {
        Lpuart3CtsBSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart3CtsBSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3CtsBSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3CtsBSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3CtsBSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART3_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3RxSelectInput(pub u32);
impl Lpuart3RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3RxSelectInput {
        Lpuart3RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart3RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART3_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3TxSelectInput(pub u32);
impl Lpuart3TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3TxSelectInput {
        Lpuart3TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart3TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART4_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4RxSelectInput(pub u32);
impl Lpuart4RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart4RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart4RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart4RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart4RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4RxSelectInput {
        Lpuart4RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart4RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart4RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART4_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4TxSelectInput(pub u32);
impl Lpuart4TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart4TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart4TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart4TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart4TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4TxSelectInput {
        Lpuart4TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart4TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart4TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART5_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5RxSelectInput(pub u32);
impl Lpuart5RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart5RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart5RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5RxSelectInput {
        Lpuart5RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart5RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart5RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART5_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart5TxSelectInput(pub u32);
impl Lpuart5TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart5TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart5TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart5TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart5TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart5TxSelectInput {
        Lpuart5TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart5TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart5TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart5TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart5TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART6_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6RxSelectInput(pub u32);
impl Lpuart6RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart6RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart6RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6RxSelectInput {
        Lpuart6RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart6RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart6RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART6_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart6TxSelectInput(pub u32);
impl Lpuart6TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart6TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart6TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart6TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart6TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart6TxSelectInput {
        Lpuart6TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart6TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart6TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart6TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart6TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART7_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart7RxSelectInput(pub u32);
impl Lpuart7RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart7RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart7RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart7RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart7RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart7RxSelectInput {
        Lpuart7RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart7RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart7RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart7RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart7RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART7_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart7TxSelectInput(pub u32);
impl Lpuart7TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart7TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart7TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart7TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart7TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart7TxSelectInput {
        Lpuart7TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart7TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart7TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart7TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart7TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART8_RX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart8RxSelectInput(pub u32);
impl Lpuart8RxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart8RxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart8RxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart8RxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart8RxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart8RxSelectInput {
        Lpuart8RxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart8RxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart8RxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart8RxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart8RxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART8_TX_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart8TxSelectInput(pub u32);
impl Lpuart8TxSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart8TxSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart8TxSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart8TxSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart8TxSelectInput {
    #[inline(always)]
    fn default() -> Lpuart8TxSelectInput {
        Lpuart8TxSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart8TxSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart8TxSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart8TxSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart8TxSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_AD_B0_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxCtl(pub u32);
impl MuxCtl {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for MuxCtl {
    #[inline(always)]
    fn default() -> MuxCtl {
        MuxCtl(0)
    }
}
impl core::fmt::Debug for MuxCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MuxCtl")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MuxCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MuxCtl {{ mux_mode: {=u8:?}, sion: {=bool:?} }}",
            self.mux_mode(),
            self.sion()
        )
    }
}
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmiSelectInput(pub u32);
impl NmiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NmiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NmiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NmiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NmiSelectInput {
    #[inline(always)]
    fn default() -> NmiSelectInput {
        NmiSelectInput(0)
    }
}
impl core::fmt::Debug for NmiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NmiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NmiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NmiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "QTIMER2_TIMER0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Timer0SelectInput(pub u32);
impl Qtimer2Timer0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Timer0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer2Timer0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Timer0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer2Timer0SelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Timer0SelectInput {
        Qtimer2Timer0SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer2Timer0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Timer0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Timer0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer2Timer0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER2_TIMER1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Timer1SelectInput(pub u32);
impl Qtimer2Timer1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Timer1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer2Timer1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Timer1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer2Timer1SelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Timer1SelectInput {
        Qtimer2Timer1SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer2Timer1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Timer1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Timer1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer2Timer1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER2_TIMER2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Timer2SelectInput(pub u32);
impl Qtimer2Timer2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Timer2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer2Timer2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Timer2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer2Timer2SelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Timer2SelectInput {
        Qtimer2Timer2SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer2Timer2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Timer2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Timer2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer2Timer2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER2_TIMER3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer2Timer3SelectInput(pub u32);
impl Qtimer2Timer3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer2Timer3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer2Timer3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer2Timer3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Qtimer2Timer3SelectInput {
    #[inline(always)]
    fn default() -> Qtimer2Timer3SelectInput {
        Qtimer2Timer3SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer2Timer3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer2Timer3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer2Timer3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer2Timer3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER3_TIMER0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Timer0SelectInput(pub u32);
impl Qtimer3Timer0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Timer0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Timer0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Timer0SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Timer0SelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Timer0SelectInput {
        Qtimer3Timer0SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer3Timer0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Timer0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Timer0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer3Timer0SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER3_TIMER1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Timer1SelectInput(pub u32);
impl Qtimer3Timer1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Timer1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Timer1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Timer1SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Timer1SelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Timer1SelectInput {
        Qtimer3Timer1SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer3Timer1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Timer1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Timer1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer3Timer1SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER3_TIMER2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Timer2SelectInput(pub u32);
impl Qtimer3Timer2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Timer2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Timer2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Timer2SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Timer2SelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Timer2SelectInput {
        Qtimer3Timer2SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer3Timer2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Timer2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Timer2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer3Timer2SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "QTIMER3_TIMER3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qtimer3Timer3SelectInput(pub u32);
impl Qtimer3Timer3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Qtimer3Timer3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Qtimer3Timer3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Qtimer3Timer3SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Qtimer3Timer3SelectInput {
    #[inline(always)]
    fn default() -> Qtimer3Timer3SelectInput {
        Qtimer3Timer3SelectInput(0)
    }
}
impl core::fmt::Debug for Qtimer3Timer3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qtimer3Timer3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qtimer3Timer3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Qtimer3Timer3SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI1_MCLK2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1Mclk2SelectInput(pub u32);
impl Sai1Mclk2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1Mclk2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1Mclk2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1Mclk2SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1Mclk2SelectInput {
    #[inline(always)]
    fn default() -> Sai1Mclk2SelectInput {
        Sai1Mclk2SelectInput(0)
    }
}
impl core::fmt::Debug for Sai1Mclk2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1Mclk2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1Mclk2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1Mclk2SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_BCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxBclkSelectInput(pub u32);
impl Sai1RxBclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxBclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1RxBclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxBclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1RxBclkSelectInput {
    #[inline(always)]
    fn default() -> Sai1RxBclkSelectInput {
        Sai1RxBclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxBclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxBclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxBclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxBclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_DATA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxData0SelectInput(pub u32);
impl Sai1RxData0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxData0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1RxData0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxData0SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1RxData0SelectInput {
    #[inline(always)]
    fn default() -> Sai1RxData0SelectInput {
        Sai1RxData0SelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxData0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxData0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxData0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxData0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_DATA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxData1SelectInput(pub u32);
impl Sai1RxData1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxData1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1RxData1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxData1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1RxData1SelectInput {
    #[inline(always)]
    fn default() -> Sai1RxData1SelectInput {
        Sai1RxData1SelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxData1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxData1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxData1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxData1SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_DATA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxData2SelectInput(pub u32);
impl Sai1RxData2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxData2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1RxData2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxData2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1RxData2SelectInput {
    #[inline(always)]
    fn default() -> Sai1RxData2SelectInput {
        Sai1RxData2SelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxData2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxData2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxData2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxData2SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_DATA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxData3SelectInput(pub u32);
impl Sai1RxData3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxData3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai1RxData3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxData3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai1RxData3SelectInput {
    #[inline(always)]
    fn default() -> Sai1RxData3SelectInput {
        Sai1RxData3SelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxData3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxData3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxData3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxData3SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_RX_SYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1RxSyncSelectInput(pub u32);
impl Sai1RxSyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1RxSyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1RxSyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1RxSyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1RxSyncSelectInput {
    #[inline(always)]
    fn default() -> Sai1RxSyncSelectInput {
        Sai1RxSyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai1RxSyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1RxSyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1RxSyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1RxSyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_TX_BCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1TxBclkSelectInput(pub u32);
impl Sai1TxBclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1TxBclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1TxBclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1TxBclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1TxBclkSelectInput {
    #[inline(always)]
    fn default() -> Sai1TxBclkSelectInput {
        Sai1TxBclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai1TxBclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1TxBclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1TxBclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1TxBclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI1_TX_SYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1TxSyncSelectInput(pub u32);
impl Sai1TxSyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai1TxSyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai1TxSyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai1TxSyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Sai1TxSyncSelectInput {
    #[inline(always)]
    fn default() -> Sai1TxSyncSelectInput {
        Sai1TxSyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai1TxSyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1TxSyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1TxSyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1TxSyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_MCLK2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2Mclk2SelectInput(pub u32);
impl Sai2Mclk2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2Mclk2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2Mclk2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2Mclk2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2Mclk2SelectInput {
    #[inline(always)]
    fn default() -> Sai2Mclk2SelectInput {
        Sai2Mclk2SelectInput(0)
    }
}
impl core::fmt::Debug for Sai2Mclk2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2Mclk2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2Mclk2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2Mclk2SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_RX_BCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2RxBclkSelectInput(pub u32);
impl Sai2RxBclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2RxBclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2RxBclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2RxBclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2RxBclkSelectInput {
    #[inline(always)]
    fn default() -> Sai2RxBclkSelectInput {
        Sai2RxBclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai2RxBclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2RxBclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2RxBclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2RxBclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_RX_DATA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2RxData0SelectInput(pub u32);
impl Sai2RxData0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2RxData0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2RxData0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2RxData0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2RxData0SelectInput {
    #[inline(always)]
    fn default() -> Sai2RxData0SelectInput {
        Sai2RxData0SelectInput(0)
    }
}
impl core::fmt::Debug for Sai2RxData0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2RxData0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2RxData0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2RxData0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_RX_SYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2RxSyncSelectInput(pub u32);
impl Sai2RxSyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2RxSyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2RxSyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2RxSyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2RxSyncSelectInput {
    #[inline(always)]
    fn default() -> Sai2RxSyncSelectInput {
        Sai2RxSyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai2RxSyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2RxSyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2RxSyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2RxSyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_TX_BCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2TxBclkSelectInput(pub u32);
impl Sai2TxBclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2TxBclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2TxBclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2TxBclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2TxBclkSelectInput {
    #[inline(always)]
    fn default() -> Sai2TxBclkSelectInput {
        Sai2TxBclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai2TxBclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2TxBclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2TxBclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2TxBclkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI2_TX_SYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2TxSyncSelectInput(pub u32);
impl Sai2TxSyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai2TxSyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai2TxSyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai2TxSyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai2TxSyncSelectInput {
    #[inline(always)]
    fn default() -> Sai2TxSyncSelectInput {
        Sai2TxSyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai2TxSyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2TxSyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2TxSyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai2TxSyncSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SAI3_IPG_CLK_SAI_MCLK_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IpgClkSaiMclkSelectInput2(pub u32);
impl Sai3IpgClkSaiMclkSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IpgClkSaiMclkSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IpgClkSaiMclkSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IpgClkSaiMclkSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IpgClkSaiMclkSelectInput2 {
    #[inline(always)]
    fn default() -> Sai3IpgClkSaiMclkSelectInput2 {
        Sai3IpgClkSaiMclkSelectInput2(0)
    }
}
impl core::fmt::Debug for Sai3IpgClkSaiMclkSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IpgClkSaiMclkSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IpgClkSaiMclkSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IpgClkSaiMclkSelectInput2 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IppIndSaiRxbclkSelectInput(pub u32);
impl Sai3IppIndSaiRxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IppIndSaiRxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IppIndSaiRxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IppIndSaiRxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IppIndSaiRxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai3IppIndSaiRxbclkSelectInput {
        Sai3IppIndSaiRxbclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai3IppIndSaiRxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IppIndSaiRxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IppIndSaiRxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IppIndSaiRxbclkSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXDATA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IppIndSaiRxdataSelectInput0(pub u32);
impl Sai3IppIndSaiRxdataSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IppIndSaiRxdataSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IppIndSaiRxdataSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IppIndSaiRxdataSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IppIndSaiRxdataSelectInput0 {
    #[inline(always)]
    fn default() -> Sai3IppIndSaiRxdataSelectInput0 {
        Sai3IppIndSaiRxdataSelectInput0(0)
    }
}
impl core::fmt::Debug for Sai3IppIndSaiRxdataSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IppIndSaiRxdataSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IppIndSaiRxdataSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IppIndSaiRxdataSelectInput0 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI3_IPP_IND_SAI_RXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IppIndSaiRxsyncSelectInput(pub u32);
impl Sai3IppIndSaiRxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IppIndSaiRxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IppIndSaiRxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IppIndSaiRxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IppIndSaiRxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai3IppIndSaiRxsyncSelectInput {
        Sai3IppIndSaiRxsyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai3IppIndSaiRxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IppIndSaiRxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IppIndSaiRxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IppIndSaiRxsyncSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI3_IPP_IND_SAI_TXBCLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IppIndSaiTxbclkSelectInput(pub u32);
impl Sai3IppIndSaiTxbclkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IppIndSaiTxbclkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IppIndSaiTxbclkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IppIndSaiTxbclkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IppIndSaiTxbclkSelectInput {
    #[inline(always)]
    fn default() -> Sai3IppIndSaiTxbclkSelectInput {
        Sai3IppIndSaiTxbclkSelectInput(0)
    }
}
impl core::fmt::Debug for Sai3IppIndSaiTxbclkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IppIndSaiTxbclkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IppIndSaiTxbclkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IppIndSaiTxbclkSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SAI3_IPP_IND_SAI_TXSYNC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3IppIndSaiTxsyncSelectInput(pub u32);
impl Sai3IppIndSaiTxsyncSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Sai3IppIndSaiTxsyncSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sai3IppIndSaiTxsyncSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Sai3IppIndSaiTxsyncSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Sai3IppIndSaiTxsyncSelectInput {
    #[inline(always)]
    fn default() -> Sai3IppIndSaiTxsyncSelectInput {
        Sai3IppIndSaiTxsyncSelectInput(0)
    }
}
impl core::fmt::Debug for Sai3IppIndSaiTxsyncSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3IppIndSaiTxsyncSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3IppIndSaiTxsyncSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai3IppIndSaiTxsyncSelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SEMC_I_IPP_IND_DQS4_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SemcIIppIndDqs4SelectInput(pub u32);
impl SemcIIppIndDqs4SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::SemcIIppIndDqs4SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SemcIIppIndDqs4SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::SemcIIppIndDqs4SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SemcIIppIndDqs4SelectInput {
    #[inline(always)]
    fn default() -> SemcIIppIndDqs4SelectInput {
        SemcIIppIndDqs4SelectInput(0)
    }
}
impl core::fmt::Debug for SemcIIppIndDqs4SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SemcIIppIndDqs4SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SemcIIppIndDqs4SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SemcIIppIndDqs4SelectInput {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "SPDIF_IN_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdifInSelectInput(pub u32);
impl SpdifInSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::SpdifInSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SpdifInSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::SpdifInSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for SpdifInSelectInput {
    #[inline(always)]
    fn default() -> SpdifInSelectInput {
        SpdifInSelectInput(0)
    }
}
impl core::fmt::Debug for SpdifInSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpdifInSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpdifInSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpdifInSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USB_OTG1_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg1OcSelectInput(pub u32);
impl UsbOtg1OcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbOtg1OcSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbOtg1OcSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbOtg1OcSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtg1OcSelectInput {
    #[inline(always)]
    fn default() -> UsbOtg1OcSelectInput {
        UsbOtg1OcSelectInput(0)
    }
}
impl core::fmt::Debug for UsbOtg1OcSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg1OcSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtg1OcSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbOtg1OcSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USB_OTG2_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtg2OcSelectInput(pub u32);
impl UsbOtg2OcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbOtg2OcSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbOtg2OcSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbOtg2OcSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtg2OcSelectInput {
    #[inline(always)]
    fn default() -> UsbOtg2OcSelectInput {
        UsbOtg2OcSelectInput(0)
    }
}
impl core::fmt::Debug for UsbOtg2OcSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtg2OcSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtg2OcSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbOtg2OcSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC1_CD_B_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc1CdBSelectInput(pub u32);
impl Usdhc1CdBSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc1CdBSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Usdhc1CdBSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc1CdBSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Usdhc1CdBSelectInput {
    #[inline(always)]
    fn default() -> Usdhc1CdBSelectInput {
        Usdhc1CdBSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc1CdBSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc1CdBSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc1CdBSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc1CdBSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC1_WP_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc1WpSelectInput(pub u32);
impl Usdhc1WpSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc1WpSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Usdhc1WpSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc1WpSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Usdhc1WpSelectInput {
    #[inline(always)]
    fn default() -> Usdhc1WpSelectInput {
        Usdhc1WpSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc1WpSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc1WpSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc1WpSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc1WpSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_CD_B_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2CdBSelectInput(pub u32);
impl Usdhc2CdBSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2CdBSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2CdBSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2CdBSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2CdBSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2CdBSelectInput {
        Usdhc2CdBSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2CdBSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2CdBSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2CdBSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2CdBSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_CLK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2ClkSelectInput(pub u32);
impl Usdhc2ClkSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2ClkSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2ClkSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2ClkSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2ClkSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2ClkSelectInput {
        Usdhc2ClkSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2ClkSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2ClkSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2ClkSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2ClkSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_CMD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2CmdSelectInput(pub u32);
impl Usdhc2CmdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2CmdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2CmdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2CmdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2CmdSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2CmdSelectInput {
        Usdhc2CmdSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2CmdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2CmdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2CmdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2CmdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA0_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data0SelectInput(pub u32);
impl Usdhc2Data0SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data0SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data0SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data0SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data0SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data0SelectInput {
        Usdhc2Data0SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data0SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data0SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data0SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data0SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data1SelectInput(pub u32);
impl Usdhc2Data1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data1SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data1SelectInput {
        Usdhc2Data1SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data1SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data2SelectInput(pub u32);
impl Usdhc2Data2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data2SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data2SelectInput {
        Usdhc2Data2SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data2SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA3_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data3SelectInput(pub u32);
impl Usdhc2Data3SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data3SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data3SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data3SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data3SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data3SelectInput {
        Usdhc2Data3SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data3SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data3SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data3SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data3SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA4_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data4SelectInput(pub u32);
impl Usdhc2Data4SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data4SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data4SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data4SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data4SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data4SelectInput {
        Usdhc2Data4SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data4SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data4SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data4SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data4SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA5_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data5SelectInput(pub u32);
impl Usdhc2Data5SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data5SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data5SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data5SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data5SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data5SelectInput {
        Usdhc2Data5SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data5SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data5SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data5SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data5SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA6_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data6SelectInput(pub u32);
impl Usdhc2Data6SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data6SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data6SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data6SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data6SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data6SelectInput {
        Usdhc2Data6SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data6SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data6SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data6SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data6SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_DATA7_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2Data7SelectInput(pub u32);
impl Usdhc2Data7SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2Data7SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2Data7SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2Data7SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2Data7SelectInput {
    #[inline(always)]
    fn default() -> Usdhc2Data7SelectInput {
        Usdhc2Data7SelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2Data7SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2Data7SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2Data7SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2Data7SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USDHC2_WP_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc2WpSelectInput(pub u32);
impl Usdhc2WpSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Usdhc2WpSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usdhc2WpSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Usdhc2WpSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Usdhc2WpSelectInput {
    #[inline(always)]
    fn default() -> Usdhc2WpSelectInput {
        Usdhc2WpSelectInput(0)
    }
}
impl core::fmt::Debug for Usdhc2WpSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usdhc2WpSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usdhc2WpSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usdhc2WpSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN02_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In02SelectInput(pub u32);
impl Xbar1In02SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In02SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In02SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In02SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In02SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In02SelectInput {
        Xbar1In02SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In02SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In02SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In02SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In02SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN03_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In03SelectInput(pub u32);
impl Xbar1In03SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In03SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In03SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In03SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In03SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In03SelectInput {
        Xbar1In03SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In03SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In03SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In03SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In03SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN04_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In04SelectInput(pub u32);
impl Xbar1In04SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In04SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In04SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In04SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In04SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In04SelectInput {
        Xbar1In04SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In04SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In04SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In04SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In04SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN05_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In05SelectInput(pub u32);
impl Xbar1In05SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In05SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In05SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In05SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In05SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In05SelectInput {
        Xbar1In05SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In05SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In05SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In05SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In05SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN06_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In06SelectInput(pub u32);
impl Xbar1In06SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In06SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In06SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In06SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In06SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In06SelectInput {
        Xbar1In06SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In06SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In06SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In06SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In06SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN07_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In07SelectInput(pub u32);
impl Xbar1In07SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In07SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In07SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In07SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In07SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In07SelectInput {
        Xbar1In07SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In07SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In07SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In07SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In07SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN08_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In08SelectInput(pub u32);
impl Xbar1In08SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In08SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In08SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In08SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In08SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In08SelectInput {
        Xbar1In08SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In08SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In08SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In08SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In08SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN09_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In09SelectInput(pub u32);
impl Xbar1In09SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In09SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In09SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In09SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In09SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In09SelectInput {
        Xbar1In09SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In09SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In09SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In09SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In09SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN14_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In14SelectInput(pub u32);
impl Xbar1In14SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In14SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In14SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In14SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In14SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In14SelectInput {
        Xbar1In14SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In14SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In14SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In14SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In14SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN15_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In15SelectInput(pub u32);
impl Xbar1In15SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In15SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In15SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In15SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In15SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In15SelectInput {
        Xbar1In15SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In15SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In15SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In15SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In15SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN16_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In16SelectInput(pub u32);
impl Xbar1In16SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In16SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In16SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In16SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In16SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In16SelectInput {
        Xbar1In16SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In16SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In16SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In16SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In16SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN17_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In17SelectInput(pub u32);
impl Xbar1In17SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In17SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Xbar1In17SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In17SelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Xbar1In17SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In17SelectInput {
        Xbar1In17SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In17SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In17SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In17SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In17SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN18_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In18SelectInput(pub u32);
impl Xbar1In18SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In18SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In18SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In18SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In18SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In18SelectInput {
        Xbar1In18SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In18SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In18SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In18SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In18SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN19_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In19SelectInput(pub u32);
impl Xbar1In19SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In19SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In19SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In19SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In19SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In19SelectInput {
        Xbar1In19SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In19SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In19SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In19SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In19SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN20_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In20SelectInput(pub u32);
impl Xbar1In20SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In20SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In20SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In20SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In20SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In20SelectInput {
        Xbar1In20SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In20SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In20SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In20SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In20SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In21SelectInput(pub u32);
impl Xbar1In21SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In21SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In21SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In21SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In21SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In21SelectInput {
        Xbar1In21SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In21SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In21SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In21SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In21SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN22_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In22SelectInput(pub u32);
impl Xbar1In22SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In22SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In22SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In22SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In22SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In22SelectInput {
        Xbar1In22SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In22SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In22SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In22SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In22SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN23_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In23SelectInput(pub u32);
impl Xbar1In23SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In23SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In23SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In23SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In23SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In23SelectInput {
        Xbar1In23SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In23SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In23SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In23SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In23SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN24_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In24SelectInput(pub u32);
impl Xbar1In24SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In24SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In24SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In24SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In24SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In24SelectInput {
        Xbar1In24SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In24SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In24SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In24SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In24SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XBAR1_IN25_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar1In25SelectInput(pub u32);
impl Xbar1In25SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Xbar1In25SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xbar1In25SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Xbar1In25SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Xbar1In25SelectInput {
    #[inline(always)]
    fn default() -> Xbar1In25SelectInput {
        Xbar1In25SelectInput(0)
    }
}
impl core::fmt::Debug for Xbar1In25SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xbar1In25SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xbar1In25SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xbar1In25SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
