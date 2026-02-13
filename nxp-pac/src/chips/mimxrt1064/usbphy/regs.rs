#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[must_use]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("enauto_pwron_pll", &self.enauto_pwron_pll())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("enidchg_wkup", &self.enidchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
            .field("rsvd1", &self.rsvd1())
            .field("otg_id_value", &self.otg_id_value())
            .field("host_force_ls_se0", &self.host_force_ls_se0())
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
            "Ctrl {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, enauto_pwron_pll: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, enidchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, fsdll_rst_en: {=bool:?}, rsvd1: {=u8:?}, otg_id_value: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
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
            self.enauto_pwron_pll(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.enidchg_wkup(),
            self.envbuschg_wkup(),
            self.fsdll_rst_en(),
            self.rsvd1(),
            self.otg_id_value(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[must_use]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
            .field("enauto_pwron_pll", &self.enauto_pwron_pll())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("enidchg_wkup", &self.enidchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
            .field("rsvd1", &self.rsvd1())
            .field("otg_id_value", &self.otg_id_value())
            .field("host_force_ls_se0", &self.host_force_ls_se0())
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
            "CtrlClr {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, enauto_pwron_pll: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, enidchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, fsdll_rst_en: {=bool:?}, rsvd1: {=u8:?}, otg_id_value: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
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
            self.enauto_pwron_pll(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.enidchg_wkup(),
            self.envbuschg_wkup(),
            self.fsdll_rst_en(),
            self.rsvd1(),
            self.otg_id_value(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[must_use]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
            .field("enauto_pwron_pll", &self.enauto_pwron_pll())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("enidchg_wkup", &self.enidchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
            .field("rsvd1", &self.rsvd1())
            .field("otg_id_value", &self.otg_id_value())
            .field("host_force_ls_se0", &self.host_force_ls_se0())
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
            "CtrlSet {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, enauto_pwron_pll: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, enidchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, fsdll_rst_en: {=bool:?}, rsvd1: {=u8:?}, otg_id_value: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
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
            self.enauto_pwron_pll(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.enidchg_wkup(),
            self.envbuschg_wkup(),
            self.fsdll_rst_en(),
            self.rsvd1(),
            self.otg_id_value(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates that the device is connected"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected"]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level3"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level3"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that there is a wakeup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enauto_pwron_pll(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub const fn set_enauto_pwron_pll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn enidchg_wkup(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_enidchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[must_use]
    #[inline(always)]
    pub const fn fsdll_rst_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub const fn set_fsdll_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub const fn set_host_force_ls_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
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
            .field("enauto_pwron_pll", &self.enauto_pwron_pll())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("enidchg_wkup", &self.enidchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
            .field("rsvd1", &self.rsvd1())
            .field("otg_id_value", &self.otg_id_value())
            .field("host_force_ls_se0", &self.host_force_ls_se0())
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
            "CtrlTog {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, enauto_pwron_pll: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, enidchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, fsdll_rst_en: {=bool:?}, rsvd1: {=u8:?}, otg_id_value: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
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
            self.enauto_pwron_pll(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.enidchg_wkup(),
            self.envbuschg_wkup(),
            self.fsdll_rst_en(),
            self.rsvd1(),
            self.otg_id_value(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug(pub u32);
impl Debug {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[must_use]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[must_use]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[must_use]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Debug {
    #[inline(always)]
    fn default() -> Debug {
        Debug(0)
    }
}
impl core::fmt::Debug for Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("rsvd0", &self.rsvd0())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("rsvd1", &self.rsvd1())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("rsvd2", &self.rsvd2())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug {{ otgidpiolock: {=bool:?}, debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, rsvd0: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, rsvd1: {=u8:?}, squelchresetcount: {=u8:?}, rsvd2: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?}, rsvd3: {=bool:?} }}",
            self.otgidpiolock(),
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.rsvd0(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.rsvd1(),
            self.squelchresetcount(),
            self.rsvd2(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate(),
            self.rsvd3()
        )
    }
}
#[doc = "UTMI Debug Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Status(pub u32);
impl Debug0Status {
    #[doc = "Running count of the failed pseudo-random generator loopback"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_back_fail_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Running count of the failed pseudo-random generator loopback"]
    #[inline(always)]
    pub const fn set_loop_back_fail_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Running count of the UTMI_RXERROR."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_rxerror_fail_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Running count of the UTMI_RXERROR."]
    #[inline(always)]
    pub const fn set_utmi_rxerror_fail_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Running count of the squelch reset instead of normal end for HS RX."]
    #[must_use]
    #[inline(always)]
    pub const fn squelch_count(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "Running count of the squelch reset instead of normal end for HS RX."]
    #[inline(always)]
    pub const fn set_squelch_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Debug0Status {
    #[inline(always)]
    fn default() -> Debug0Status {
        Debug0Status(0)
    }
}
impl core::fmt::Debug for Debug0Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Status")
            .field("loop_back_fail_count", &self.loop_back_fail_count())
            .field("utmi_rxerror_fail_count", &self.utmi_rxerror_fail_count())
            .field("squelch_count", &self.squelch_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Status {{ loop_back_fail_count: {=u16:?}, utmi_rxerror_fail_count: {=u16:?}, squelch_count: {=u8:?} }}",
            self.loop_back_fail_count(),
            self.utmi_rxerror_fail_count(),
            self.squelch_count()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1(pub u32);
impl Debug1 {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1 {
    #[inline(always)]
    fn default() -> Debug1 {
        Debug1(0)
    }
}
impl core::fmt::Debug for Debug1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug1")
            .field("rsvd0", &self.rsvd0())
            .field("entailadjvd", &self.entailadjvd())
            .field("rsvd1", &self.rsvd1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1 {{ rsvd0: {=u16:?}, entailadjvd: {=u8:?}, rsvd1: {=u32:?} }}",
            self.rsvd0(),
            self.entailadjvd(),
            self.rsvd1()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Clr(pub u32);
impl Debug1Clr {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1Clr {
    #[inline(always)]
    fn default() -> Debug1Clr {
        Debug1Clr(0)
    }
}
impl core::fmt::Debug for Debug1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug1Clr")
            .field("rsvd0", &self.rsvd0())
            .field("entailadjvd", &self.entailadjvd())
            .field("rsvd1", &self.rsvd1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Clr {{ rsvd0: {=u16:?}, entailadjvd: {=u8:?}, rsvd1: {=u32:?} }}",
            self.rsvd0(),
            self.entailadjvd(),
            self.rsvd1()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Set(pub u32);
impl Debug1Set {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1Set {
    #[inline(always)]
    fn default() -> Debug1Set {
        Debug1Set(0)
    }
}
impl core::fmt::Debug for Debug1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug1Set")
            .field("rsvd0", &self.rsvd0())
            .field("entailadjvd", &self.entailadjvd())
            .field("rsvd1", &self.rsvd1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Set {{ rsvd0: {=u16:?}, entailadjvd: {=u8:?}, rsvd1: {=u32:?} }}",
            self.rsvd0(),
            self.entailadjvd(),
            self.rsvd1()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Tog(pub u32);
impl Debug1Tog {
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Delay increment of the rise of squelch: 00 = Delay is nominal 01 = Delay is +20% 10 = Delay is -20% 11 = Delay is -40%"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u32 {
        let val = (self.0 >> 15usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 15usize)) | (((val as u32) & 0x0001_ffff) << 15usize);
    }
}
impl Default for Debug1Tog {
    #[inline(always)]
    fn default() -> Debug1Tog {
        Debug1Tog(0)
    }
}
impl core::fmt::Debug for Debug1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug1Tog")
            .field("rsvd0", &self.rsvd0())
            .field("entailadjvd", &self.entailadjvd())
            .field("rsvd1", &self.rsvd1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Tog {{ rsvd0: {=u16:?}, entailadjvd: {=u8:?}, rsvd1: {=u32:?} }}",
            self.rsvd0(),
            self.entailadjvd(),
            self.rsvd1()
        )
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugClr(pub u32);
impl DebugClr {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[must_use]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[must_use]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[must_use]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugClr {
    #[inline(always)]
    fn default() -> DebugClr {
        DebugClr(0)
    }
}
impl core::fmt::Debug for DebugClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugClr")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("rsvd0", &self.rsvd0())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("rsvd1", &self.rsvd1())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("rsvd2", &self.rsvd2())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugClr {{ otgidpiolock: {=bool:?}, debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, rsvd0: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, rsvd1: {=u8:?}, squelchresetcount: {=u8:?}, rsvd2: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?}, rsvd3: {=bool:?} }}",
            self.otgidpiolock(),
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.rsvd0(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.rsvd1(),
            self.squelchresetcount(),
            self.rsvd2(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate(),
            self.rsvd3()
        )
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugSet(pub u32);
impl DebugSet {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[must_use]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[must_use]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[must_use]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugSet {
    #[inline(always)]
    fn default() -> DebugSet {
        DebugSet(0)
    }
}
impl core::fmt::Debug for DebugSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugSet")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("rsvd0", &self.rsvd0())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("rsvd1", &self.rsvd1())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("rsvd2", &self.rsvd2())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugSet {{ otgidpiolock: {=bool:?}, debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, rsvd0: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, rsvd1: {=u8:?}, squelchresetcount: {=u8:?}, rsvd2: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?}, rsvd3: {=bool:?} }}",
            self.otgidpiolock(),
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.rsvd0(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.rsvd1(),
            self.squelchresetcount(),
            self.rsvd2(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate(),
            self.rsvd3()
        )
    }
}
#[doc = "USB PHY Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugTog(pub u32);
impl DebugTog {
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_interface_hold(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub const fn set_debug_interface_hold(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[must_use]
    #[inline(always)]
    pub const fn tx2rxcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub const fn set_tx2rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[must_use]
    #[inline(always)]
    pub const fn entx2rxcount(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub const fn set_entx2rxcount(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub const fn set_squelchresetcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[must_use]
    #[inline(always)]
    pub const fn ensquelchreset(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub const fn set_ensquelchreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn squelchresetlength(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub const fn set_squelchresetlength(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn host_resume_debug(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub const fn set_host_resume_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate Test Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate Test Clocks"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DebugTog {
    #[inline(always)]
    fn default() -> DebugTog {
        DebugTog(0)
    }
}
impl core::fmt::Debug for DebugTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugTog")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("rsvd0", &self.rsvd0())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("rsvd1", &self.rsvd1())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("rsvd2", &self.rsvd2())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .field("rsvd3", &self.rsvd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugTog {{ otgidpiolock: {=bool:?}, debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, rsvd0: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, rsvd1: {=u8:?}, squelchresetcount: {=u8:?}, rsvd2: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?}, rsvd3: {=bool:?} }}",
            self.otgidpiolock(),
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.rsvd0(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.rsvd1(),
            self.squelchresetcount(),
            self.rsvd2(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate(),
            self.rsvd3()
        )
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwd(pub u32);
impl Pwd {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rsvd1", &self.rsvd1())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwd {{ rsvd0: {=u16:?}, txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rsvd1: {=u8:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?}, rsvd2: {=u16:?} }}",
            self.rsvd0(),
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rsvd1(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdClr(pub u32);
impl PwdClr {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rsvd1", &self.rsvd1())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdClr {{ rsvd0: {=u16:?}, txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rsvd1: {=u8:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?}, rsvd2: {=u16:?} }}",
            self.rsvd0(),
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rsvd1(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdSet(pub u32);
impl PwdSet {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rsvd1", &self.rsvd1())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdSet {{ rsvd0: {=u16:?}, txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rsvd1: {=u8:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?}, rsvd2: {=u16:?} }}",
            self.rsvd0(),
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rsvd1(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdTog(pub u32);
impl PwdTog {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x07ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rsvd1", &self.rsvd1())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdTog {{ rsvd0: {=u16:?}, txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rsvd1: {=u8:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?}, rsvd2: {=u16:?} }}",
            self.rsvd0(),
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rsvd1(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("disconadj", &self.disconadj())
            .field("rsvd1", &self.rsvd1())
            .field("rxdbypass", &self.rxdbypass())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rx {{ envadj: {=u8:?}, rsvd0: {=bool:?}, disconadj: {=u8:?}, rsvd1: {=u16:?}, rxdbypass: {=bool:?}, rsvd2: {=u16:?} }}",
            self.envadj(),
            self.rsvd0(),
            self.disconadj(),
            self.rsvd1(),
            self.rxdbypass(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxClr(pub u32);
impl RxClr {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("disconadj", &self.disconadj())
            .field("rsvd1", &self.rsvd1())
            .field("rxdbypass", &self.rxdbypass())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxClr {{ envadj: {=u8:?}, rsvd0: {=bool:?}, disconadj: {=u8:?}, rsvd1: {=u16:?}, rxdbypass: {=bool:?}, rsvd2: {=u16:?} }}",
            self.envadj(),
            self.rsvd0(),
            self.disconadj(),
            self.rsvd1(),
            self.rxdbypass(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxSet(pub u32);
impl RxSet {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("disconadj", &self.disconadj())
            .field("rsvd1", &self.rsvd1())
            .field("rxdbypass", &self.rxdbypass())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxSet {{ envadj: {=u8:?}, rsvd0: {=bool:?}, disconadj: {=u8:?}, rsvd1: {=u16:?}, rxdbypass: {=bool:?}, rsvd2: {=u16:?} }}",
            self.envadj(),
            self.rsvd0(),
            self.disconadj(),
            self.rsvd1(),
            self.rxdbypass(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxTog(pub u32);
impl RxTog {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector: 000 = Trip-Level Voltage is 0"]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x7fff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 7usize)) | (((val as u32) & 0x7fff) << 7usize);
    }
    #[doc = "0 = Normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "0 = Normal operation"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u16 {
        let val = (self.0 >> 23usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("disconadj", &self.disconadj())
            .field("rsvd1", &self.rsvd1())
            .field("rxdbypass", &self.rxdbypass())
            .field("rsvd2", &self.rsvd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxTog {{ envadj: {=u8:?}, rsvd0: {=bool:?}, disconadj: {=u8:?}, rsvd1: {=u16:?}, rxdbypass: {=bool:?}, rsvd2: {=u16:?} }}",
            self.envadj(),
            self.rsvd0(),
            self.disconadj(),
            self.rsvd1(),
            self.rxdbypass(),
            self.rsvd2()
        )
    }
}
#[doc = "USB PHY Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Indicates that the device has disconnected while in high-speed host mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected while in high-speed host mode."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[inline(always)]
    pub const fn set_devplugin_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates the results of ID pin on MiniAB plug"]
    #[must_use]
    #[inline(always)]
    pub const fn otgid_status(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub const fn set_otgid_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[inline(always)]
    pub const fn set_resume_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd4(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("hostdiscondetect_status", &self.hostdiscondetect_status())
            .field("rsvd1", &self.rsvd1())
            .field("devplugin_status", &self.devplugin_status())
            .field("rsvd2", &self.rsvd2())
            .field("otgid_status", &self.otgid_status())
            .field("rsvd3", &self.rsvd3())
            .field("resume_status", &self.resume_status())
            .field("rsvd4", &self.rsvd4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ rsvd0: {=u8:?}, hostdiscondetect_status: {=bool:?}, rsvd1: {=u8:?}, devplugin_status: {=bool:?}, rsvd2: {=bool:?}, otgid_status: {=bool:?}, rsvd3: {=bool:?}, resume_status: {=bool:?}, rsvd4: {=u32:?} }}",
            self.rsvd0(),
            self.hostdiscondetect_status(),
            self.rsvd1(),
            self.devplugin_status(),
            self.rsvd2(),
            self.otgid_status(),
            self.rsvd3(),
            self.resume_status(),
            self.rsvd4()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx(pub u32);
impl Tx {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txcal45dn", &self.txcal45dn())
            .field("rsvd1", &self.rsvd1())
            .field("txcal45dp", &self.txcal45dp())
            .field("rsvd2", &self.rsvd2())
            .field("usbphy_tx_edgectrl", &self.usbphy_tx_edgectrl())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tx {{ d_cal: {=u8:?}, rsvd0: {=u8:?}, txcal45dn: {=u8:?}, rsvd1: {=u8:?}, txcal45dp: {=u8:?}, rsvd2: {=u8:?}, usbphy_tx_edgectrl: {=u8:?}, rsvd5: {=u8:?} }}",
            self.d_cal(),
            self.rsvd0(),
            self.txcal45dn(),
            self.rsvd1(),
            self.txcal45dp(),
            self.rsvd2(),
            self.usbphy_tx_edgectrl(),
            self.rsvd5()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxClr(pub u32);
impl TxClr {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txcal45dn", &self.txcal45dn())
            .field("rsvd1", &self.rsvd1())
            .field("txcal45dp", &self.txcal45dp())
            .field("rsvd2", &self.rsvd2())
            .field("usbphy_tx_edgectrl", &self.usbphy_tx_edgectrl())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxClr {{ d_cal: {=u8:?}, rsvd0: {=u8:?}, txcal45dn: {=u8:?}, rsvd1: {=u8:?}, txcal45dp: {=u8:?}, rsvd2: {=u8:?}, usbphy_tx_edgectrl: {=u8:?}, rsvd5: {=u8:?} }}",
            self.d_cal(),
            self.rsvd0(),
            self.txcal45dn(),
            self.rsvd1(),
            self.txcal45dp(),
            self.rsvd2(),
            self.usbphy_tx_edgectrl(),
            self.rsvd5()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxSet(pub u32);
impl TxSet {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txcal45dn", &self.txcal45dn())
            .field("rsvd1", &self.rsvd1())
            .field("txcal45dp", &self.txcal45dp())
            .field("rsvd2", &self.rsvd2())
            .field("usbphy_tx_edgectrl", &self.usbphy_tx_edgectrl())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxSet {{ d_cal: {=u8:?}, rsvd0: {=u8:?}, txcal45dn: {=u8:?}, rsvd1: {=u8:?}, txcal45dp: {=u8:?}, rsvd2: {=u8:?}, usbphy_tx_edgectrl: {=u8:?}, rsvd5: {=u8:?} }}",
            self.d_cal(),
            self.rsvd0(),
            self.txcal45dn(),
            self.rsvd1(),
            self.txcal45dp(),
            self.rsvd2(),
            self.usbphy_tx_edgectrl(),
            self.rsvd5()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxTog(pub u32);
impl TxTog {
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub const fn set_rsvd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_edgectrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub const fn set_usbphy_tx_edgectrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd5(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_rsvd5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
            .field("rsvd0", &self.rsvd0())
            .field("txcal45dn", &self.txcal45dn())
            .field("rsvd1", &self.rsvd1())
            .field("txcal45dp", &self.txcal45dp())
            .field("rsvd2", &self.rsvd2())
            .field("usbphy_tx_edgectrl", &self.usbphy_tx_edgectrl())
            .field("rsvd5", &self.rsvd5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxTog {{ d_cal: {=u8:?}, rsvd0: {=u8:?}, txcal45dn: {=u8:?}, rsvd1: {=u8:?}, txcal45dp: {=u8:?}, rsvd2: {=u8:?}, usbphy_tx_edgectrl: {=u8:?}, rsvd5: {=u8:?} }}",
            self.d_cal(),
            self.rsvd0(),
            self.txcal45dn(),
            self.rsvd1(),
            self.txcal45dp(),
            self.rsvd2(),
            self.usbphy_tx_edgectrl(),
            self.rsvd5()
        )
    }
}
#[doc = "UTMI RTL Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fixed read-only value reflecting the stepping of the RTL version."]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL version."]
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
