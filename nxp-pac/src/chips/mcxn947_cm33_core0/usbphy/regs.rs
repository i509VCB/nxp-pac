#[doc = "Analog Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl(pub u32);
impl Anactrl {
    #[doc = "Internal Low Voltage Detector Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> super::vals::LviEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LviEn::from_bits(val as u8)
    }
    #[doc = "Internal Low Voltage Detector Enable"]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: super::vals::LviEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> super::vals::PfdClkSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PfdClkSel::from_bits(val as u8)
    }
    #[doc = "PFD Clock Selection"]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: super::vals::PfdClkSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> super::vals::DevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DevPulldown::from_bits(val as u8)
    }
    #[doc = "Device Pulldown Enable"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: super::vals::DevPulldown) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Anactrl {
    #[inline(always)]
    fn default() -> Anactrl {
        Anactrl(0)
    }
}
impl core::fmt::Debug for Anactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anactrl")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Anactrl {{ lvi_en: {:?}, pfd_clk_sel: {:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlClr(pub u32);
impl AnactrlClr {
    #[doc = "Internal Low Voltage Detector Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable"]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection"]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlClr {
    #[inline(always)]
    fn default() -> AnactrlClr {
        AnactrlClr(0)
    }
}
impl core::fmt::Debug for AnactrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlClr")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlClr {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlSet(pub u32);
impl AnactrlSet {
    #[doc = "Internal Low Voltage Detector Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable"]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection"]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlSet {
    #[inline(always)]
    fn default() -> AnactrlSet {
        AnactrlSet(0)
    }
}
impl core::fmt::Debug for AnactrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlSet")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlSet {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlTog(pub u32);
impl AnactrlTog {
    #[doc = "Internal Low Voltage Detector Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable"]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection"]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlTog {
    #[inline(always)]
    fn default() -> AnactrlTog {
        AnactrlTog(0)
    }
}
impl core::fmt::Debug for AnactrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlTog")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlTog {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "General Purpose Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "OTG ID Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> super::vals::HostdiscondetectIrq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::HostdiscondetectIrq::from_bits(val as u8)
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: super::vals::HostdiscondetectIrq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> super::vals::Endevplugindetect {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Endevplugindetect::from_bits(val as u8)
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: super::vals::Endevplugindetect) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> super::vals::DevpluginPolarity {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DevpluginPolarity::from_bits(val as u8)
    }
    #[doc = "Device Plug-In Polarity"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: super::vals::DevpluginPolarity) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt"]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> super::vals::Enotgiddetect {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Enotgiddetect::from_bits(val as u8)
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: super::vals::Enotgiddetect) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option"]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option"]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> super::vals::OtgIdValue {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OtgIdValue::from_bits(val as u8)
    }
    #[doc = "OTG ID Value"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: super::vals::OtgIdValue) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> super::vals::Clkgate {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Clkgate::from_bits(val as u8)
    }
    #[doc = "UTMI Clock Gate"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: super::vals::Clkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> super::vals::Sftrst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Sftrst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: super::vals::Sftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {:?}, endevplugindetect: {:?}, devplugin_polarity: {:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {:?}, utmi_suspendm: {=bool:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "OTG ID Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt"]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option"]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option"]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlClr {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "OTG ID Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt"]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option"]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option"]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlSet {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "OTG ID Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt"]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector"]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option"]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option"]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlTog {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Debug 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0(pub u32);
impl Debug0 {
    #[doc = "Hold OTG_ID"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> super::vals::Hstpulldown {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Hstpulldown::from_bits(val as u8)
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: super::vals::Hstpulldown) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> super::vals::Enhstpulldown {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Enhstpulldown::from_bits(val as u8)
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: super::vals::Enhstpulldown) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0 {
    #[inline(always)]
    fn default() -> Debug0 {
        Debug0(0)
    }
}
impl core::fmt::Debug for Debug0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0 {{ otgidpiolock: {=bool:?}, hstpulldown: {:?}, enhstpulldown: {:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Clr(pub u32);
impl Debug0Clr {
    #[doc = "Hold OTG_ID"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Clr {
    #[inline(always)]
    fn default() -> Debug0Clr {
        Debug0Clr(0)
    }
}
impl core::fmt::Debug for Debug0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Clr")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Clr {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Set(pub u32);
impl Debug0Set {
    #[doc = "Hold OTG_ID"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Set {
    #[inline(always)]
    fn default() -> Debug0Set {
        Debug0Set(0)
    }
}
impl core::fmt::Debug for Debug0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Set")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Set {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Tog(pub u32);
impl Debug0Tog {
    #[doc = "Hold OTG_ID"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Tog {
    #[inline(always)]
    fn default() -> Debug0Tog {
        Debug0Tog(0)
    }
}
impl core::fmt::Debug for Debug0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Tog")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Tog {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "IP Block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ip(pub u32);
impl Ip {
    #[doc = "Power Control Suspend Option"]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option"]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ip {
    #[inline(always)]
    fn default() -> Ip {
        Ip(0)
    }
}
impl core::fmt::Debug for Ip {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ip")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ip {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ip {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpClr(pub u32);
impl IpClr {
    #[doc = "Power Control Suspend Option"]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option"]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpClr {
    #[inline(always)]
    fn default() -> IpClr {
        IpClr(0)
    }
}
impl core::fmt::Debug for IpClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpClr")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpClr {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpSet(pub u32);
impl IpSet {
    #[doc = "Power Control Suspend Option"]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option"]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpSet {
    #[inline(always)]
    fn default() -> IpSet {
        IpSet(0)
    }
}
impl core::fmt::Debug for IpSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpSet")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpSet {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpTog(pub u32);
impl IpTog {
    #[doc = "Power Control Suspend Option"]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option"]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpTog {
    #[inline(always)]
    fn default() -> IpTog {
        IpTog(0)
    }
}
impl core::fmt::Debug for IpTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpTog")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpTog {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "PFD A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfda(pub u32);
impl Pfda {
    #[doc = "PFD0 Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> super::vals::Pfd0Clkgate {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pfd0Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD0 Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: super::vals::Pfd0Clkgate) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pfda {
    #[inline(always)]
    fn default() -> Pfda {
        Pfda(0)
    }
}
impl core::fmt::Debug for Pfda {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfda")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfda {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfda {{ pfd0_clkgate: {:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaClr(pub u32);
impl PfdaClr {
    #[doc = "PFD0 Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaClr {
    #[inline(always)]
    fn default() -> PfdaClr {
        PfdaClr(0)
    }
}
impl core::fmt::Debug for PfdaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaClr")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaClr {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaSet(pub u32);
impl PfdaSet {
    #[doc = "PFD0 Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaSet {
    #[inline(always)]
    fn default() -> PfdaSet {
        PfdaSet(0)
    }
}
impl core::fmt::Debug for PfdaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaSet")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaSet {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaTog(pub u32);
impl PfdaTog {
    #[doc = "PFD0 Clock Gate"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider"]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal"]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaTog {
    #[inline(always)]
    fn default() -> PfdaTog {
        PfdaTog(0)
    }
}
impl core::fmt::Debug for PfdaTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaTog")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaTog {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PLL SIC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSic(pub u32);
impl PllSic {
    #[doc = "Miscellaneous Control"]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> super::vals::Misc2Control0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Misc2Control0::from_bits(val as u8)
    }
    #[doc = "Miscellaneous Control"]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: super::vals::Misc2Control0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> super::vals::PllEnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PllEnUsbClks::from_bits(val as u8)
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: super::vals::PllEnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> super::vals::PllPower {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PllPower::from_bits(val as u8)
    }
    #[doc = "USB PLL Powerup Control"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: super::vals::PllPower) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> super::vals::PllEnable {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PllEnable::from_bits(val as u8)
    }
    #[doc = "PLL Output Clock Enable"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: super::vals::PllEnable) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> super::vals::PllBypass {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PllBypass::from_bits(val as u8)
    }
    #[doc = "Bypass USB PLL"]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: super::vals::PllBypass) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> super::vals::RefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::RefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference Bias Power Control"]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: super::vals::RefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> super::vals::RefbiasPwd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::RefbiasPwd::from_bits(val as u8)
    }
    #[doc = "Power Down Reference Bias"]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: super::vals::RefbiasPwd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> super::vals::PllRegEnable {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PllRegEnable::from_bits(val as u8)
    }
    #[doc = "Enable PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: super::vals::PllRegEnable) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> super::vals::PllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PllDivSel::from_bits(val as u8)
    }
    #[doc = "PLL Divider Value Configuration"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: super::vals::PllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> super::vals::PllLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: super::vals::PllLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSic {
    #[inline(always)]
    fn default() -> PllSic {
        PllSic(0)
    }
}
impl core::fmt::Debug for PllSic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSic")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSic {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSic {{ misc2_control0: {:?}, pll_en_usb_clks: {:?}, pll_power: {:?}, pll_enable: {:?}, pll_bypass: {:?}, refbias_pwd_sel: {:?}, refbias_pwd: {:?}, pll_reg_enable: {:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicClr(pub u32);
impl PllSicClr {
    #[doc = "Miscellaneous Control"]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control"]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL"]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control"]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias"]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicClr {
    #[inline(always)]
    fn default() -> PllSicClr {
        PllSicClr(0)
    }
}
impl core::fmt::Debug for PllSicClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicClr")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicClr {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicSet(pub u32);
impl PllSicSet {
    #[doc = "Miscellaneous Control"]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control"]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL"]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control"]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias"]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicSet {
    #[inline(always)]
    fn default() -> PllSicSet {
        PllSicSet(0)
    }
}
impl core::fmt::Debug for PllSicSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicSet")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicSet {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicTog(pub u32);
impl PllSicTog {
    #[doc = "Miscellaneous Control"]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control"]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL"]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control"]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias"]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias"]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicTog {
    #[inline(always)]
    fn default() -> PllSicTog {
        PllSicTog(0)
    }
}
impl core::fmt::Debug for PllSicTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicTog")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicTog {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "Power Down"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwd(pub u32);
impl Pwd {
    #[doc = "Power Down USB FS TX Drivers"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> super::vals::Txpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Txpwdfs::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS TX Drivers"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: super::vals::Txpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> super::vals::Txpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Txpwdibias::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: super::vals::Txpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> super::vals::Txpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Txpwdv2i::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: super::vals::Txpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> super::vals::Rxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rxpwdenv::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: super::vals::Rxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> super::vals::Rxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Rxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: super::vals::Rxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> super::vals::Rxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Rxpwddiff::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: super::vals::Rxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> super::vals::Rxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Rxpwdrx::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: super::vals::Rxpwdrx) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Pwd {
    #[inline(always)]
    fn default() -> Pwd {
        Pwd(0)
    }
}
impl core::fmt::Debug for Pwd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwd")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwd {{ txpwdfs: {:?}, txpwdibias: {:?}, txpwdv2i: {:?}, rxpwdenv: {:?}, rxpwd1pt1: {:?}, rxpwddiff: {:?}, rxpwdrx: {:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdClr(pub u32);
impl PwdClr {
    #[doc = "Power Down USB FS TX Drivers"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdClr {
    #[inline(always)]
    fn default() -> PwdClr {
        PwdClr(0)
    }
}
impl core::fmt::Debug for PwdClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdClr")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdClr {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdSet(pub u32);
impl PwdSet {
    #[doc = "Power Down USB FS TX Drivers"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdSet {
    #[inline(always)]
    fn default() -> PwdSet {
        PwdSet(0)
    }
}
impl core::fmt::Debug for PwdSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdSet")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdSet {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdTog(pub u32);
impl PwdTog {
    #[doc = "Power Down USB FS TX Drivers"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdTog {
    #[inline(always)]
    fn default() -> PwdTog {
        PwdTog(0)
    }
}
impl core::fmt::Debug for PwdTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdTog")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdTog {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "RX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "Envelope Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> super::vals::Envadj {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Envadj::from_bits(val as u8)
    }
    #[doc = "Envelope Detector Trip Point"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: super::vals::Envadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> super::vals::Disconadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Disconadj::from_bits(val as u8)
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: super::vals::Disconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for Rx {
    #[inline(always)]
    fn default() -> Rx {
        Rx(0)
    }
}
impl core::fmt::Debug for Rx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rx {{ envadj: {:?}, disconadj: {:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxClr(pub u32);
impl RxClr {
    #[doc = "Envelope Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxClr {
    #[inline(always)]
    fn default() -> RxClr {
        RxClr(0)
    }
}
impl core::fmt::Debug for RxClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxClr")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxClr {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxSet(pub u32);
impl RxSet {
    #[doc = "Envelope Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxSet {
    #[inline(always)]
    fn default() -> RxSet {
        RxSet(0)
    }
}
impl core::fmt::Debug for RxSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxSet")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxSet {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxTog(pub u32);
impl RxTog {
    #[doc = "Envelope Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxTog {
    #[inline(always)]
    fn default() -> RxTog {
        RxTog(0)
    }
}
impl core::fmt::Debug for RxTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxTog")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxTog {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "USB 3.3 V and 1.8 V Supply Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ok_status_3v(&self) -> super::vals::OkStatus3v {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::OkStatus3v::from_bits(val as u8)
    }
    #[doc = "USB 3.3 V and 1.8 V Supply Status"]
    #[inline(always)]
    pub const fn set_ok_status_3v(&mut self, val: super::vals::OkStatus3v) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Status"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Status"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_status(&self) -> super::vals::DevpluginStatus {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DevpluginStatus::from_bits(val as u8)
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection"]
    #[inline(always)]
    pub const fn set_devplugin_status(&mut self, val: super::vals::DevpluginStatus) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "OTG ID Status"]
    #[must_use]
    #[inline(always)]
    pub const fn otgid_status(&self) -> super::vals::OtgidStatus {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OtgidStatus::from_bits(val as u8)
    }
    #[doc = "OTG ID Status"]
    #[inline(always)]
    pub const fn set_otgid_status(&mut self, val: super::vals::OtgidStatus) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Status"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Status"]
    #[inline(always)]
    pub const fn set_resume_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("ok_status_3v", &self.ok_status_3v())
            .field("hostdiscondetect_status", &self.hostdiscondetect_status())
            .field("devplugin_status", &self.devplugin_status())
            .field("otgid_status", &self.otgid_status())
            .field("resume_status", &self.resume_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ ok_status_3v: {:?}, hostdiscondetect_status: {=bool:?}, devplugin_status: {:?}, otgid_status: {:?}, resume_status: {=bool:?} }}",
            self.ok_status_3v(),
            self.hostdiscondetect_status(),
            self.devplugin_status(),
            self.otgid_status(),
            self.resume_status()
        )
    }
}
#[doc = "Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEn(pub u32);
impl TrimOverrideEn {
    #[doc = "Override Enable for PLL Divider Value"]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> super::vals::DivSelOverride {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DivSelOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for PLL Divider Value"]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: super::vals::DivSelOverride) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> super::vals::TxDCalOverride {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TxDCalOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: super::vals::TxDCalOverride) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> super::vals::TxCal45dpOverride {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TxCal45dpOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: super::vals::TxCal45dpOverride) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> super::vals::TxCal45dmOverride {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TxCal45dmOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: super::vals::TxCal45dmOverride) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> super::vals::UsbphyTxDCal {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::UsbphyTxDCal::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: super::vals::UsbphyTxDCal) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEn {
    #[inline(always)]
    fn default() -> TrimOverrideEn {
        TrimOverrideEn(0)
    }
}
impl core::fmt::Debug for TrimOverrideEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEn")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEn {{ div_sel_override: {:?}, tx_d_cal_override: {:?}, tx_cal45dp_override: {:?}, tx_cal45dm_override: {:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnClr(pub u32);
impl TrimOverrideEnClr {
    #[doc = "Override Enable for PLL Divider Value"]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value"]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnClr {
    #[inline(always)]
    fn default() -> TrimOverrideEnClr {
        TrimOverrideEnClr(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnClr")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnClr {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnSet(pub u32);
impl TrimOverrideEnSet {
    #[doc = "Override Enable for PLL Divider Value"]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value"]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnSet {
    #[inline(always)]
    fn default() -> TrimOverrideEnSet {
        TrimOverrideEnSet(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnSet")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnSet {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnTog(pub u32);
impl TrimOverrideEnTog {
    #[doc = "Override Enable for PLL Divider Value"]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value"]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim"]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY"]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnTog {
    #[inline(always)]
    fn default() -> TrimOverrideEnTog {
        TrimOverrideEnTog(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnTog")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnTog {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "TX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx(pub u32);
impl Tx {
    #[doc = "HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> super::vals::DCal {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DCal::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: super::vals::DCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tx {
    #[inline(always)]
    fn default() -> Tx {
        Tx(0)
    }
}
impl core::fmt::Debug for Tx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tx")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tx {{ d_cal: {:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxClr(pub u32);
impl TxClr {
    #[doc = "HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxClr {
    #[inline(always)]
    fn default() -> TxClr {
        TxClr(0)
    }
}
impl core::fmt::Debug for TxClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxClr")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxClr {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxSet(pub u32);
impl TxSet {
    #[doc = "HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxSet {
    #[inline(always)]
    fn default() -> TxSet {
        TxSet(0)
    }
}
impl core::fmt::Debug for TxSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxSet")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxSet {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxTog(pub u32);
impl TxTog {
    #[doc = "HS TX Output Current Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxTog {
    #[inline(always)]
    fn default() -> TxTog {
        TxTog(0)
    }
}
impl core::fmt::Debug for TxTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxTog")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxTog {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "Charger Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStat(pub u32);
impl Usb1ChrgDetStat {
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> super::vals::PlugContact {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PlugContact::from_bits(val as u8)
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: super::vals::PlugContact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> super::vals::ChrgDetected {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChrgDetected::from_bits(val as u8)
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: super::vals::ChrgDetected) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> super::vals::DmState {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DmState::from_bits(val as u8)
    }
    #[doc = "DM Voltage"]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: super::vals::DmState) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> super::vals::DpState {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DpState::from_bits(val as u8)
    }
    #[doc = "DP Voltage"]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: super::vals::DpState) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> super::vals::SecdetDcp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SecdetDcp::from_bits(val as u8)
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: super::vals::SecdetDcp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStat {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStat {
        Usb1ChrgDetStat(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStat {{ plug_contact: {:?}, chrg_detected: {:?}, dm_state: {:?}, dp_state: {:?}, secdet_dcp: {:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatClr(pub u32);
impl Usb1ChrgDetStatClr {
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage"]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage"]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatClr {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatClr {
        Usb1ChrgDetStatClr(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatClr")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatClr {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatSet(pub u32);
impl Usb1ChrgDetStatSet {
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage"]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage"]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatSet {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatSet {
        Usb1ChrgDetStatSet(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatSet")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatSet {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatTog(pub u32);
impl Usb1ChrgDetStatTog {
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output"]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage"]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage"]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output"]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatTog {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatTog {
        Usb1ChrgDetStatTog(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatTog")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatTog {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetect(pub u32);
impl Usb1ChrgDetect {
    #[doc = "Secondary Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> super::vals::DetectSec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DetectSec::from_bits(val as u8)
    }
    #[doc = "Secondary Detection Function Enable"]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: super::vals::DetectSec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> super::vals::PullupDp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PullupDp::from_bits(val as u8)
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: super::vals::PullupDp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> super::vals::VdmSrcEnable {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VdmSrcEnable::from_bits(val as u8)
    }
    #[doc = "VDM_SRC Function Enable"]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: super::vals::VdmSrcEnable) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> super::vals::ChkContact {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ChkContact::from_bits(val as u8)
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: super::vals::ChkContact) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> super::vals::ChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ChkChrgB::from_bits(val as u8)
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: super::vals::ChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> super::vals::EnB {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::EnB::from_bits(val as u8)
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: super::vals::EnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> super::vals::Dcdsel {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dcdsel::from_bits(val as u8)
    }
    #[doc = "DCD Selection"]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: super::vals::Dcdsel) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetect {
    #[inline(always)]
    fn default() -> Usb1ChrgDetect {
        Usb1ChrgDetect(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetect")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetect {{ detect_sec: {:?}, pullup_dp: {:?}, vdm_src_enable: {:?}, chk_contact: {:?}, chk_chrg_b: {:?}, en_b: {:?}, dcdsel: {:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectClr(pub u32);
impl Usb1ChrgDetectClr {
    #[doc = "Secondary Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable"]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable"]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection"]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectClr {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectClr {
        Usb1ChrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectClr")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectClr {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectSet(pub u32);
impl Usb1ChrgDetectSet {
    #[doc = "Secondary Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable"]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable"]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection"]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectSet {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectSet {
        Usb1ChrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectSet")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectSet {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectTog(pub u32);
impl Usb1ChrgDetectTog {
    #[doc = "Secondary Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable"]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control"]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable"]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable"]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable"]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable"]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection"]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectTog {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectTog {
        Usb1ChrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectTog")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectTog {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "VBUS Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStat(pub u32);
impl Usb1VbusDetStat {
    #[doc = "Session End Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> super::vals::Sessend {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sessend::from_bits(val as u8)
    }
    #[doc = "Session End Indicator"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: super::vals::Sessend) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> super::vals::Bvalid {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bvalid::from_bits(val as u8)
    }
    #[doc = "B-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: super::vals::Bvalid) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> super::vals::Avalid {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Avalid::from_bits(val as u8)
    }
    #[doc = "A-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: super::vals::Avalid) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> super::vals::VbusValid {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VbusValid::from_bits(val as u8)
    }
    #[doc = "VBUS Voltage Status"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: super::vals::VbusValid) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> super::vals::VbusValid3v {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VbusValid3v::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: super::vals::VbusValid3v) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status"]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStat {
    #[inline(always)]
    fn default() -> Usb1VbusDetStat {
        Usb1VbusDetStat(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStat {{ sessend: {:?}, bvalid: {:?}, avalid: {:?}, vbus_valid: {:?}, vbus_valid_3v: {:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatClr(pub u32);
impl Usb1VbusDetStatClr {
    #[doc = "Session End Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status"]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatClr {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatClr {
        Usb1VbusDetStatClr(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatClr")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatClr {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatSet(pub u32);
impl Usb1VbusDetStatSet {
    #[doc = "Session End Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status"]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatSet {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatSet {
        Usb1VbusDetStatSet(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatSet")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatSet {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatTog(pub u32);
impl Usb1VbusDetStatTog {
    #[doc = "Session End Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status"]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status"]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatTog {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatTog {
        Usb1VbusDetStatTog(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatTog")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatTog {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetect(pub u32);
impl Usb1VbusDetect {
    #[doc = "VBUS Comparator Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::VbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::VbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "VBUS Comparator Threshold"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::VbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> super::vals::VbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: super::vals::VbusOverrideEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> super::vals::VbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::VbusvalidSel::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: super::vals::VbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::VbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::VbusSourceSel::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::VbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> super::vals::IdOverrideEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::IdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: super::vals::IdOverrideEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override"]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> super::vals::ExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::ExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "External ID Override Enable"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: super::vals::ExtIdOverrideEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> super::vals::ExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::ExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "External VBUS Override Enable"]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: super::vals::ExtVbusOverrideEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> super::vals::VbusvalidToB {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::VbusvalidToB::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: super::vals::VbusvalidToB) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> super::vals::VbusvalidPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::VbusvalidPwrupCmps::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: super::vals::VbusvalidPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> super::vals::DischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DischargeVbus::from_bits(val as u8)
    }
    #[doc = "VBUS Discharge Resistor"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: super::vals::DischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetect {
    #[inline(always)]
    fn default() -> Usb1VbusDetect {
        Usb1VbusDetect(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetect {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_b: {:?}, vbusvalid_pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectClr(pub u32);
impl Usb1VbusDetectClr {
    #[doc = "VBUS Comparator Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override"]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable"]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectClr {
    #[inline(always)]
    fn default() -> Usb1VbusDetectClr {
        Usb1VbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectClr {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectSet(pub u32);
impl Usb1VbusDetectSet {
    #[doc = "VBUS Comparator Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override"]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable"]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectSet {
    #[inline(always)]
    fn default() -> Usb1VbusDetectSet {
        Usb1VbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectSet {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectTog(pub u32);
impl Usb1VbusDetectTog {
    #[doc = "VBUS Comparator Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override"]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override"]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override"]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable"]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable"]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectTog {
    #[inline(always)]
    fn default() -> Usb1VbusDetectTog {
        Usb1VbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectTog {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Step"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Step"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ step: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.step(),
            self.minor(),
            self.major()
        )
    }
}
