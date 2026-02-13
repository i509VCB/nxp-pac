#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl(pub u32);
impl Anactrl {
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> super::vals::AnactrlDevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AnactrlDevPulldown::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: super::vals::AnactrlDevPulldown) {
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
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Anactrl {{ dev_pulldown: {:?} }}", self.dev_pulldown())
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlClr(pub u32);
impl AnactrlClr {
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> super::vals::AnactrlClrDevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AnactrlClrDevPulldown::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: super::vals::AnactrlClrDevPulldown) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlClr {{ dev_pulldown: {:?} }}",
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlSet(pub u32);
impl AnactrlSet {
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> super::vals::AnactrlSetDevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AnactrlSetDevPulldown::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: super::vals::AnactrlSetDevPulldown) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlSet {{ dev_pulldown: {:?} }}",
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlTog(pub u32);
impl AnactrlTog {
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> super::vals::AnactrlTogDevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AnactrlTogDevPulldown::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: super::vals::AnactrlTogDevPulldown) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlTog {{ dev_pulldown: {:?} }}",
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY General Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
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
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindet(&self) -> super::vals::CtrlEndevplugindet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtrlEndevplugindet::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub const fn set_endevplugindet(&mut self, val: super::vals::CtrlEndevplugindet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
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
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
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
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
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
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
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
            "Ctrl {{ enhostdiscondetect: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, fsdll_rst_en: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.fsdll_rst_en(),
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
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindet(&self) -> super::vals::CtrlClrEndevplugindet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtrlClrEndevplugindet::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub const fn set_endevplugindet(&mut self, val: super::vals::CtrlClrEndevplugindet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
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
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
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
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
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
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
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
            "CtrlClr {{ enhostdiscondetect: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, fsdll_rst_en: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.fsdll_rst_en(),
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
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindet(&self) -> super::vals::CtrlSetEndevplugindet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtrlSetEndevplugindet::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub const fn set_endevplugindet(&mut self, val: super::vals::CtrlSetEndevplugindet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
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
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
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
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
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
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
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
            "CtrlSet {{ enhostdiscondetect: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, fsdll_rst_en: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.fsdll_rst_en(),
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
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindet(&self) -> super::vals::CtrlTogEndevplugindet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CtrlTogEndevplugindet::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub const fn set_endevplugindet(&mut self, val: super::vals::CtrlTogEndevplugindet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
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
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[must_use]
    #[inline(always)]
    pub const fn host_force_ls_se0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
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
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
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
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("fsdll_rst_en", &self.fsdll_rst_en())
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
            "CtrlTog {{ enhostdiscondetect: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, fsdll_rst_en: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.fsdll_rst_en(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "USB PHY Debug Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0(pub u32);
impl Debug0 {
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
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0 {{ debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, squelchresetcount: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?} }}",
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.squelchresetcount(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate()
        )
    }
}
#[doc = "USB PHY Debug Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Clr(pub u32);
impl Debug0Clr {
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
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Clr {{ debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, squelchresetcount: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?} }}",
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.squelchresetcount(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate()
        )
    }
}
#[doc = "USB PHY Debug Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Set(pub u32);
impl Debug0Set {
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
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Set {{ debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, squelchresetcount: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?} }}",
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.squelchresetcount(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate()
        )
    }
}
#[doc = "USB PHY Debug Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Tog(pub u32);
impl Debug0Tog {
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
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
            .field("debug_interface_hold", &self.debug_interface_hold())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .field("tx2rxcount", &self.tx2rxcount())
            .field("entx2rxcount", &self.entx2rxcount())
            .field("squelchresetcount", &self.squelchresetcount())
            .field("ensquelchreset", &self.ensquelchreset())
            .field("squelchresetlength", &self.squelchresetlength())
            .field("host_resume_debug", &self.host_resume_debug())
            .field("clkgate", &self.clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Tog {{ debug_interface_hold: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?}, tx2rxcount: {=u8:?}, entx2rxcount: {=bool:?}, squelchresetcount: {=u8:?}, ensquelchreset: {=bool:?}, squelchresetlength: {=u8:?}, host_resume_debug: {=bool:?}, clkgate: {=bool:?} }}",
            self.debug_interface_hold(),
            self.hstpulldown(),
            self.enhstpulldown(),
            self.tx2rxcount(),
            self.entx2rxcount(),
            self.squelchresetcount(),
            self.ensquelchreset(),
            self.squelchresetlength(),
            self.host_resume_debug(),
            self.clkgate()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1(pub u32);
impl Debug1 {
    #[doc = "Delay increment of the rise of squelch:"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> super::vals::Debug1Entailadjvd {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Debug1Entailadjvd::from_bits(val as u8)
    }
    #[doc = "Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: super::vals::Debug1Entailadjvd) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Adjustment bits on bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits on bandgap"]
    #[inline(always)]
    pub const fn set_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
            .field("entailadjvd", &self.entailadjvd())
            .field("usb2_refbias_vbgadj", &self.usb2_refbias_vbgadj())
            .field("usb2_refbias_tst", &self.usb2_refbias_tst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1 {{ entailadjvd: {:?}, usb2_refbias_vbgadj: {=u8:?}, usb2_refbias_tst: {=u8:?} }}",
            self.entailadjvd(),
            self.usb2_refbias_vbgadj(),
            self.usb2_refbias_tst()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Clr(pub u32);
impl Debug1Clr {
    #[doc = "Delay increment of the rise of squelch:"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> super::vals::Debug1ClrEntailadjvd {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Debug1ClrEntailadjvd::from_bits(val as u8)
    }
    #[doc = "Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: super::vals::Debug1ClrEntailadjvd) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Adjustment bits on bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits on bandgap"]
    #[inline(always)]
    pub const fn set_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
            .field("entailadjvd", &self.entailadjvd())
            .field("usb2_refbias_vbgadj", &self.usb2_refbias_vbgadj())
            .field("usb2_refbias_tst", &self.usb2_refbias_tst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Clr {{ entailadjvd: {:?}, usb2_refbias_vbgadj: {=u8:?}, usb2_refbias_tst: {=u8:?} }}",
            self.entailadjvd(),
            self.usb2_refbias_vbgadj(),
            self.usb2_refbias_tst()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Set(pub u32);
impl Debug1Set {
    #[doc = "Delay increment of the rise of squelch:"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> super::vals::Debug1SetEntailadjvd {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Debug1SetEntailadjvd::from_bits(val as u8)
    }
    #[doc = "Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: super::vals::Debug1SetEntailadjvd) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Adjustment bits on bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits on bandgap"]
    #[inline(always)]
    pub const fn set_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
            .field("entailadjvd", &self.entailadjvd())
            .field("usb2_refbias_vbgadj", &self.usb2_refbias_vbgadj())
            .field("usb2_refbias_tst", &self.usb2_refbias_tst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Set {{ entailadjvd: {:?}, usb2_refbias_vbgadj: {=u8:?}, usb2_refbias_tst: {=u8:?} }}",
            self.entailadjvd(),
            self.usb2_refbias_vbgadj(),
            self.usb2_refbias_tst()
        )
    }
}
#[doc = "UTMI Debug Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug1Tog(pub u32);
impl Debug1Tog {
    #[doc = "Delay increment of the rise of squelch:"]
    #[must_use]
    #[inline(always)]
    pub const fn entailadjvd(&self) -> super::vals::Debug1TogEntailadjvd {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Debug1TogEntailadjvd::from_bits(val as u8)
    }
    #[doc = "Delay increment of the rise of squelch:"]
    #[inline(always)]
    pub const fn set_entailadjvd(&mut self, val: super::vals::Debug1TogEntailadjvd) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Adjustment bits on bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits on bandgap"]
    #[inline(always)]
    pub const fn set_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
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
            .field("entailadjvd", &self.entailadjvd())
            .field("usb2_refbias_vbgadj", &self.usb2_refbias_vbgadj())
            .field("usb2_refbias_tst", &self.usb2_refbias_tst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug1Tog {{ entailadjvd: {:?}, usb2_refbias_vbgadj: {=u8:?}, usb2_refbias_tst: {=u8:?} }}",
            self.entailadjvd(),
            self.usb2_refbias_vbgadj(),
            self.usb2_refbias_tst()
        )
    }
}
#[doc = "USB PHY PLL Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSic(pub u32);
impl PllSic {
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> super::vals::PllSicRefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PllSicRefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: super::vals::PllSicRefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> super::vals::PllSicPllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PllSicPllDivSel::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: super::vals::PllSicPllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL lock status indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> super::vals::PllSicPllLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllSicPllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: super::vals::PllSicPllLock) {
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
            "PllSic {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
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
#[doc = "USB PHY PLL Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicClr(pub u32);
impl PllSicClr {
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> super::vals::PllSicClrRefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PllSicClrRefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: super::vals::PllSicClrRefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> super::vals::PllSicClrPllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PllSicClrPllDivSel::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: super::vals::PllSicClrPllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL lock status indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> super::vals::PllSicClrPllLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllSicClrPllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: super::vals::PllSicClrPllLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "PllSicClr {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
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
#[doc = "USB PHY PLL Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicSet(pub u32);
impl PllSicSet {
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> super::vals::PllSicSetRefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PllSicSetRefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: super::vals::PllSicSetRefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> super::vals::PllSicSetPllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PllSicSetPllDivSel::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: super::vals::PllSicSetPllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL lock status indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> super::vals::PllSicSetPllLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllSicSetPllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: super::vals::PllSicSetPllLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "PllSicSet {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
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
#[doc = "USB PHY PLL Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicTog(pub u32);
impl PllSicTog {
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> super::vals::PllSicTogRefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PllSicTogRefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: super::vals::PllSicTogRefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> super::vals::PllSicTogPllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PllSicTogPllDivSel::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: super::vals::PllSicTogPllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL lock status indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> super::vals::PllSicTogPllLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PllSicTogPllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator"]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: super::vals::PllSicTogPllLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "PllSicTog {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
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
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwd(pub u32);
impl Pwd {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> super::vals::PwdTxpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PwdTxpwdfs::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: super::vals::PwdTxpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> super::vals::PwdTxpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PwdTxpwdibias::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: super::vals::PwdTxpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> super::vals::PwdTxpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PwdTxpwdv2i::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: super::vals::PwdTxpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> super::vals::PwdRxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PwdRxpwdenv::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: super::vals::PwdRxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> super::vals::PwdRxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PwdRxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: super::vals::PwdRxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> super::vals::PwdRxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PwdRxpwddiff::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: super::vals::PwdRxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> super::vals::PwdRxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PwdRxpwdrx::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: super::vals::PwdRxpwdrx) {
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
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdClr(pub u32);
impl PwdClr {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> super::vals::PwdClrTxpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PwdClrTxpwdfs::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: super::vals::PwdClrTxpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> super::vals::PwdClrTxpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PwdClrTxpwdibias::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: super::vals::PwdClrTxpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> super::vals::PwdClrTxpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PwdClrTxpwdv2i::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: super::vals::PwdClrTxpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> super::vals::PwdClrRxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PwdClrRxpwdenv::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: super::vals::PwdClrRxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> super::vals::PwdClrRxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PwdClrRxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: super::vals::PwdClrRxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> super::vals::PwdClrRxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PwdClrRxpwddiff::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: super::vals::PwdClrRxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> super::vals::PwdClrRxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PwdClrRxpwdrx::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: super::vals::PwdClrRxpwdrx) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
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
            "PwdClr {{ txpwdfs: {:?}, txpwdibias: {:?}, txpwdv2i: {:?}, rxpwdenv: {:?}, rxpwd1pt1: {:?}, rxpwddiff: {:?}, rxpwdrx: {:?} }}",
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
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdSet(pub u32);
impl PwdSet {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> super::vals::PwdSetTxpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PwdSetTxpwdfs::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: super::vals::PwdSetTxpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> super::vals::PwdSetTxpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PwdSetTxpwdibias::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: super::vals::PwdSetTxpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> super::vals::PwdSetTxpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PwdSetTxpwdv2i::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: super::vals::PwdSetTxpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> super::vals::PwdSetRxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PwdSetRxpwdenv::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: super::vals::PwdSetRxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> super::vals::PwdSetRxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PwdSetRxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: super::vals::PwdSetRxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> super::vals::PwdSetRxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PwdSetRxpwddiff::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: super::vals::PwdSetRxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> super::vals::PwdSetRxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PwdSetRxpwdrx::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: super::vals::PwdSetRxpwdrx) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
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
            "PwdSet {{ txpwdfs: {:?}, txpwdibias: {:?}, txpwdv2i: {:?}, rxpwdenv: {:?}, rxpwd1pt1: {:?}, rxpwddiff: {:?}, rxpwdrx: {:?} }}",
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
#[doc = "USB PHY Power-Down Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdTog(pub u32);
impl PwdTog {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> super::vals::PwdTogTxpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PwdTogTxpwdfs::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: super::vals::PwdTogTxpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> super::vals::PwdTogTxpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PwdTogTxpwdibias::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: super::vals::PwdTogTxpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> super::vals::PwdTogTxpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PwdTogTxpwdv2i::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: super::vals::PwdTogTxpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> super::vals::PwdTogRxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PwdTogRxpwdenv::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: super::vals::PwdTogRxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> super::vals::PwdTogRxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PwdTogRxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: super::vals::PwdTogRxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> super::vals::PwdTogRxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PwdTogRxpwddiff::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: super::vals::PwdTogRxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> super::vals::PwdTogRxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PwdTogRxpwdrx::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: super::vals::PwdTogRxpwdrx) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
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
            "PwdTog {{ txpwdfs: {:?}, txpwdibias: {:?}, txpwdv2i: {:?}, rxpwdenv: {:?}, rxpwd1pt1: {:?}, rxpwddiff: {:?}, rxpwdrx: {:?} }}",
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
#[doc = "USB PHY Receiver Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> super::vals::RxEnvadj {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RxEnvadj::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: super::vals::RxEnvadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> super::vals::RxDisconadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RxDisconadj::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: super::vals::RxDisconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> super::vals::RxRxdbypass {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RxRxdbypass::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: super::vals::RxRxdbypass) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            .field("rxdbypass", &self.rxdbypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rx {{ envadj: {:?}, disconadj: {:?}, rxdbypass: {:?} }}",
            self.envadj(),
            self.disconadj(),
            self.rxdbypass()
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
    pub const fn envadj(&self) -> super::vals::RxClrEnvadj {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RxClrEnvadj::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: super::vals::RxClrEnvadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> super::vals::RxClrDisconadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RxClrDisconadj::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: super::vals::RxClrDisconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> super::vals::RxClrRxdbypass {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RxClrRxdbypass::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: super::vals::RxClrRxdbypass) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            .field("rxdbypass", &self.rxdbypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxClr {{ envadj: {:?}, disconadj: {:?}, rxdbypass: {:?} }}",
            self.envadj(),
            self.disconadj(),
            self.rxdbypass()
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
    pub const fn envadj(&self) -> super::vals::RxSetEnvadj {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RxSetEnvadj::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: super::vals::RxSetEnvadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> super::vals::RxSetDisconadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RxSetDisconadj::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: super::vals::RxSetDisconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> super::vals::RxSetRxdbypass {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RxSetRxdbypass::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: super::vals::RxSetRxdbypass) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            .field("rxdbypass", &self.rxdbypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxSet {{ envadj: {:?}, disconadj: {:?}, rxdbypass: {:?} }}",
            self.envadj(),
            self.disconadj(),
            self.rxdbypass()
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
    pub const fn envadj(&self) -> super::vals::RxTogEnvadj {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RxTogEnvadj::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector"]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: super::vals::RxTogEnvadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> super::vals::RxTogDisconadj {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RxTogDisconadj::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: super::vals::RxTogDisconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdbypass(&self) -> super::vals::RxTogRxdbypass {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RxTogRxdbypass::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver"]
    #[inline(always)]
    pub const fn set_rxdbypass(&mut self, val: super::vals::RxTogRxdbypass) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            .field("rxdbypass", &self.rxdbypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxTog {{ envadj: {:?}, disconadj: {:?}, rxdbypass: {:?} }}",
            self.envadj(),
            self.disconadj(),
            self.rxdbypass()
        )
    }
}
#[doc = "USB PHY Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_status(&self) -> super::vals::HostdiscondetectStatus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::HostdiscondetectStatus::from_bits(val as u8)
    }
    #[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode"]
    #[inline(always)]
    pub const fn set_hostdiscondetect_status(&mut self, val: super::vals::HostdiscondetectStatus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_status(&self) -> super::vals::DevpluginStatus {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DevpluginStatus::from_bits(val as u8)
    }
    #[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]"]
    #[inline(always)]
    pub const fn set_devplugin_status(&mut self, val: super::vals::DevpluginStatus) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
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
            .field("hostdiscondetect_status", &self.hostdiscondetect_status())
            .field("devplugin_status", &self.devplugin_status())
            .field("resume_status", &self.resume_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ hostdiscondetect_status: {:?}, devplugin_status: {:?}, resume_status: {=bool:?} }}",
            self.hostdiscondetect_status(),
            self.devplugin_status(),
            self.resume_status()
        )
    }
}
#[doc = "USB PHY Trim Override Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEn(pub u32);
impl TrimOverrideEn {
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_env_tail_adj_vd_override(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_env_tail_adj_vd_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_vbgadj_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[inline(always)]
    pub const fn set_trim_refbias_vbgadj_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_tst_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[inline(always)]
    pub const fn set_trim_refbias_tst_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Adjustment bits for bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits for bandgap"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dm(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dm(&mut self, val: u8) {
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
            .field("trim_div_sel_override", &self.trim_div_sel_override())
            .field(
                "trim_env_tail_adj_vd_override",
                &self.trim_env_tail_adj_vd_override(),
            )
            .field("trim_tx_d_cal_override", &self.trim_tx_d_cal_override())
            .field("trim_tx_cal45dp_override", &self.trim_tx_cal45dp_override())
            .field("trim_tx_cal45dm_override", &self.trim_tx_cal45dm_override())
            .field(
                "trim_refbias_vbgadj_override",
                &self.trim_refbias_vbgadj_override(),
            )
            .field(
                "trim_refbias_tst_override",
                &self.trim_refbias_tst_override(),
            )
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dm", &self.trim_usbphy_tx_cal45dm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEn {{ trim_div_sel_override: {=bool:?}, trim_env_tail_adj_vd_override: {=bool:?}, trim_tx_d_cal_override: {=bool:?}, trim_tx_cal45dp_override: {=bool:?}, trim_tx_cal45dm_override: {=bool:?}, trim_refbias_vbgadj_override: {=bool:?}, trim_refbias_tst_override: {=bool:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?}, trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?} }}",
            self.trim_div_sel_override(),
            self.trim_env_tail_adj_vd_override(),
            self.trim_tx_d_cal_override(),
            self.trim_tx_cal45dp_override(),
            self.trim_tx_cal45dm_override(),
            self.trim_refbias_vbgadj_override(),
            self.trim_refbias_tst_override(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_usb2_refbias_tst(),
            self.trim_pll_ctrl0_div_sel(),
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dm()
        )
    }
}
#[doc = "USB PHY Trim Override Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnClr(pub u32);
impl TrimOverrideEnClr {
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_env_tail_adj_vd_override(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_env_tail_adj_vd_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_vbgadj_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[inline(always)]
    pub const fn set_trim_refbias_vbgadj_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_tst_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[inline(always)]
    pub const fn set_trim_refbias_tst_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Adjustment bits for bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits for bandgap"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dm(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dm(&mut self, val: u8) {
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
            .field("trim_div_sel_override", &self.trim_div_sel_override())
            .field(
                "trim_env_tail_adj_vd_override",
                &self.trim_env_tail_adj_vd_override(),
            )
            .field("trim_tx_d_cal_override", &self.trim_tx_d_cal_override())
            .field("trim_tx_cal45dp_override", &self.trim_tx_cal45dp_override())
            .field("trim_tx_cal45dm_override", &self.trim_tx_cal45dm_override())
            .field(
                "trim_refbias_vbgadj_override",
                &self.trim_refbias_vbgadj_override(),
            )
            .field(
                "trim_refbias_tst_override",
                &self.trim_refbias_tst_override(),
            )
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dm", &self.trim_usbphy_tx_cal45dm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnClr {{ trim_div_sel_override: {=bool:?}, trim_env_tail_adj_vd_override: {=bool:?}, trim_tx_d_cal_override: {=bool:?}, trim_tx_cal45dp_override: {=bool:?}, trim_tx_cal45dm_override: {=bool:?}, trim_refbias_vbgadj_override: {=bool:?}, trim_refbias_tst_override: {=bool:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?}, trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?} }}",
            self.trim_div_sel_override(),
            self.trim_env_tail_adj_vd_override(),
            self.trim_tx_d_cal_override(),
            self.trim_tx_cal45dp_override(),
            self.trim_tx_cal45dm_override(),
            self.trim_refbias_vbgadj_override(),
            self.trim_refbias_tst_override(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_usb2_refbias_tst(),
            self.trim_pll_ctrl0_div_sel(),
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dm()
        )
    }
}
#[doc = "USB PHY Trim Override Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnSet(pub u32);
impl TrimOverrideEnSet {
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_env_tail_adj_vd_override(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_env_tail_adj_vd_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_vbgadj_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[inline(always)]
    pub const fn set_trim_refbias_vbgadj_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_tst_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[inline(always)]
    pub const fn set_trim_refbias_tst_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Adjustment bits for bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits for bandgap"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dm(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dm(&mut self, val: u8) {
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
            .field("trim_div_sel_override", &self.trim_div_sel_override())
            .field(
                "trim_env_tail_adj_vd_override",
                &self.trim_env_tail_adj_vd_override(),
            )
            .field("trim_tx_d_cal_override", &self.trim_tx_d_cal_override())
            .field("trim_tx_cal45dp_override", &self.trim_tx_cal45dp_override())
            .field("trim_tx_cal45dm_override", &self.trim_tx_cal45dm_override())
            .field(
                "trim_refbias_vbgadj_override",
                &self.trim_refbias_vbgadj_override(),
            )
            .field(
                "trim_refbias_tst_override",
                &self.trim_refbias_tst_override(),
            )
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dm", &self.trim_usbphy_tx_cal45dm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnSet {{ trim_div_sel_override: {=bool:?}, trim_env_tail_adj_vd_override: {=bool:?}, trim_tx_d_cal_override: {=bool:?}, trim_tx_cal45dp_override: {=bool:?}, trim_tx_cal45dm_override: {=bool:?}, trim_refbias_vbgadj_override: {=bool:?}, trim_refbias_tst_override: {=bool:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?}, trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?} }}",
            self.trim_div_sel_override(),
            self.trim_env_tail_adj_vd_override(),
            self.trim_tx_d_cal_override(),
            self.trim_tx_cal45dp_override(),
            self.trim_tx_cal45dm_override(),
            self.trim_refbias_vbgadj_override(),
            self.trim_refbias_tst_override(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_usb2_refbias_tst(),
            self.trim_pll_ctrl0_div_sel(),
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dm()
        )
    }
}
#[doc = "USB PHY Trim Override Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnTog(pub u32);
impl TrimOverrideEnTog {
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_env_tail_adj_vd_override(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_env_tail_adj_vd_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\] will be used."]
    #[inline(always)]
    pub const fn set_trim_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_vbgadj_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bandgap adjustment"]
    #[inline(always)]
    pub const fn set_trim_refbias_vbgadj_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_refbias_tst_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\] will be used"]
    #[inline(always)]
    pub const fn set_trim_refbias_tst_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Adjustment bits for bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Adjustment bits for bandgap"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Bias current control for usb2_phy"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "Bias current control for usb2_phy"]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_D_CAL."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dm(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dm(&mut self, val: u8) {
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
            .field("trim_div_sel_override", &self.trim_div_sel_override())
            .field(
                "trim_env_tail_adj_vd_override",
                &self.trim_env_tail_adj_vd_override(),
            )
            .field("trim_tx_d_cal_override", &self.trim_tx_d_cal_override())
            .field("trim_tx_cal45dp_override", &self.trim_tx_cal45dp_override())
            .field("trim_tx_cal45dm_override", &self.trim_tx_cal45dm_override())
            .field(
                "trim_refbias_vbgadj_override",
                &self.trim_refbias_vbgadj_override(),
            )
            .field(
                "trim_refbias_tst_override",
                &self.trim_refbias_tst_override(),
            )
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dm", &self.trim_usbphy_tx_cal45dm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnTog {{ trim_div_sel_override: {=bool:?}, trim_env_tail_adj_vd_override: {=bool:?}, trim_tx_d_cal_override: {=bool:?}, trim_tx_cal45dp_override: {=bool:?}, trim_tx_cal45dm_override: {=bool:?}, trim_refbias_vbgadj_override: {=bool:?}, trim_refbias_tst_override: {=bool:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?}, trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?} }}",
            self.trim_div_sel_override(),
            self.trim_env_tail_adj_vd_override(),
            self.trim_tx_d_cal_override(),
            self.trim_tx_cal45dp_override(),
            self.trim_tx_cal45dm_override(),
            self.trim_refbias_vbgadj_override(),
            self.trim_refbias_tst_override(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_usb2_refbias_tst(),
            self.trim_pll_ctrl0_div_sel(),
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dm()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx(pub u32);
impl Tx {
    #[doc = "Decode to trim the nominal 17"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> super::vals::TxDCal {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TxDCal::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: super::vals::TxDCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub const fn set_txcal45dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dn(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_txencal45dn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_txencal45dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
            .field("txcal45dm", &self.txcal45dm())
            .field("txencal45dn", &self.txencal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .field("txencal45dp", &self.txencal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tx {{ d_cal: {:?}, txcal45dm: {=u8:?}, txencal45dn: {=bool:?}, txcal45dp: {=u8:?}, txencal45dp: {=bool:?} }}",
            self.d_cal(),
            self.txcal45dm(),
            self.txencal45dn(),
            self.txcal45dp(),
            self.txencal45dp()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxClr(pub u32);
impl TxClr {
    #[doc = "Decode to trim the nominal 17"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> super::vals::TxClrDCal {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TxClrDCal::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: super::vals::TxClrDCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub const fn set_txcal45dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dn(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_txencal45dn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_txencal45dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
            .field("txcal45dm", &self.txcal45dm())
            .field("txencal45dn", &self.txencal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .field("txencal45dp", &self.txencal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxClr {{ d_cal: {:?}, txcal45dm: {=u8:?}, txencal45dn: {=bool:?}, txcal45dp: {=u8:?}, txencal45dp: {=bool:?} }}",
            self.d_cal(),
            self.txcal45dm(),
            self.txencal45dn(),
            self.txcal45dp(),
            self.txencal45dp()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxSet(pub u32);
impl TxSet {
    #[doc = "Decode to trim the nominal 17"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> super::vals::TxSetDCal {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TxSetDCal::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: super::vals::TxSetDCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub const fn set_txcal45dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dn(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_txencal45dn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_txencal45dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
            .field("txcal45dm", &self.txcal45dm())
            .field("txencal45dn", &self.txencal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .field("txencal45dp", &self.txencal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxSet {{ d_cal: {:?}, txcal45dm: {=u8:?}, txencal45dn: {=bool:?}, txcal45dp: {=u8:?}, txencal45dp: {=bool:?} }}",
            self.d_cal(),
            self.txcal45dm(),
            self.txencal45dn(),
            self.txcal45dp(),
            self.txencal45dp()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxTog(pub u32);
impl TxTog {
    #[doc = "Decode to trim the nominal 17"]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> super::vals::TxTogDCal {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TxTogDCal::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17"]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: super::vals::TxTogDCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub const fn set_txcal45dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dn(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_txencal45dn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn txencal45dp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_txencal45dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
            .field("txcal45dm", &self.txcal45dm())
            .field("txencal45dn", &self.txencal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .field("txencal45dp", &self.txencal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxTog {{ d_cal: {:?}, txcal45dm: {=u8:?}, txencal45dn: {=bool:?}, txcal45dp: {=u8:?}, txencal45dp: {=bool:?} }}",
            self.d_cal(),
            self.txcal45dm(),
            self.txencal45dn(),
            self.txcal45dp(),
            self.txencal45dp()
        )
    }
}
#[doc = "USB PHY Charger Detect Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStat(pub u32);
impl Usb1ChrgDetStat {
    #[doc = "Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> super::vals::PlugContact {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PlugContact::from_bits(val as u8)
    }
    #[doc = "Battery Charging Data Contact Detection phase output During the Data Contact Detection phase per the USB Battery Charging Specification Revision 1"]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: super::vals::PlugContact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> super::vals::ChrgDetected {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ChrgDetected::from_bits(val as u8)
    }
    #[doc = "Battery Charging Primary Detection phase output During the USB Battery Charging Primary Detection phase using the USBHSDCD module, this bit field indicates whether a Standard Downstream Port or Charging Port was detected"]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: super::vals::ChrgDetected) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> super::vals::DmState {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DmState::from_bits(val as u8)
    }
    #[doc = "Single ended receiver output for the USB_DM pin, from charger detection circuits."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: super::vals::DmState) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> super::vals::DpState {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DpState::from_bits(val as u8)
    }
    #[doc = "Single ended receiver output for the USB_DP pin, from charger detection circuits."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: super::vals::DpState) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> super::vals::SecdetDcp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SecdetDcp::from_bits(val as u8)
    }
    #[doc = "Battery Charging Secondary Detection phase output During the USB Battery Charging Secondary Detection phase using the USBHSDCD module, this bit field indicates which kind of Charging Port was detected"]
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
#[doc = "USB PHY Charger Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetect(pub u32);
impl Usb1ChrgDetect {
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[must_use]
    #[inline(always)]
    pub const fn bgr_ibias(&self) -> super::vals::Usb1ChrgDetectBgrIbias {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usb1ChrgDetectBgrIbias::from_bits(val as u8)
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub const fn set_bgr_ibias(&mut self, val: super::vals::Usb1ChrgDetectBgrIbias) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("pullup_dp", &self.pullup_dp())
            .field("bgr_ibias", &self.bgr_ibias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetect {{ pullup_dp: {=bool:?}, bgr_ibias: {:?} }}",
            self.pullup_dp(),
            self.bgr_ibias()
        )
    }
}
#[doc = "USB PHY Charger Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectClr(pub u32);
impl Usb1ChrgDetectClr {
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[must_use]
    #[inline(always)]
    pub const fn bgr_ibias(&self) -> super::vals::Usb1ChrgDetectClrBgrIbias {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usb1ChrgDetectClrBgrIbias::from_bits(val as u8)
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub const fn set_bgr_ibias(&mut self, val: super::vals::Usb1ChrgDetectClrBgrIbias) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("pullup_dp", &self.pullup_dp())
            .field("bgr_ibias", &self.bgr_ibias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectClr {{ pullup_dp: {=bool:?}, bgr_ibias: {:?} }}",
            self.pullup_dp(),
            self.bgr_ibias()
        )
    }
}
#[doc = "USB PHY Charger Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectSet(pub u32);
impl Usb1ChrgDetectSet {
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[must_use]
    #[inline(always)]
    pub const fn bgr_ibias(&self) -> super::vals::Usb1ChrgDetectSetBgrIbias {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usb1ChrgDetectSetBgrIbias::from_bits(val as u8)
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub const fn set_bgr_ibias(&mut self, val: super::vals::Usb1ChrgDetectSetBgrIbias) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("pullup_dp", &self.pullup_dp())
            .field("bgr_ibias", &self.bgr_ibias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectSet {{ pullup_dp: {=bool:?}, bgr_ibias: {:?} }}",
            self.pullup_dp(),
            self.bgr_ibias()
        )
    }
}
#[doc = "USB PHY Charger Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectTog(pub u32);
impl Usb1ChrgDetectTog {
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is used to pull up DP, for digital charge detect."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[must_use]
    #[inline(always)]
    pub const fn bgr_ibias(&self) -> super::vals::Usb1ChrgDetectTogBgrIbias {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Usb1ChrgDetectTogBgrIbias::from_bits(val as u8)
    }
    #[doc = "USB charge detector bias current reference This bit determines the reference for the bias current of the USB charge detector"]
    #[inline(always)]
    pub const fn set_bgr_ibias(&mut self, val: super::vals::Usb1ChrgDetectTogBgrIbias) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("pullup_dp", &self.pullup_dp())
            .field("bgr_ibias", &self.bgr_ibias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectTog {{ pullup_dp: {=bool:?}, bgr_ibias: {:?} }}",
            self.pullup_dp(),
            self.bgr_ibias()
        )
    }
}
#[doc = "USB PHY Loopback Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1Loopback(pub u32);
impl Usb1Loopback {
    #[doc = "This bit enables the USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the USB loopback test."]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hs_mode(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub const fn set_tsti_tx_hs_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_ls_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub const fn set_tsti_tx_ls_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable TX for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hiz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hsfs_mode_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub const fn set_tsti_hsfs_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tstpkt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub const fn set_tstpkt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Usb1Loopback {
    #[inline(always)]
    fn default() -> Usb1Loopback {
        Usb1Loopback(0)
    }
}
impl core::fmt::Debug for Usb1Loopback {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1Loopback")
            .field("utmi_teststart", &self.utmi_teststart())
            .field("utmi_dig_tst0", &self.utmi_dig_tst0())
            .field("utmi_dig_tst1", &self.utmi_dig_tst1())
            .field("tsti_tx_hs_mode", &self.tsti_tx_hs_mode())
            .field("tsti_tx_ls_mode", &self.tsti_tx_ls_mode())
            .field("tsti_tx_en", &self.tsti_tx_en())
            .field("tsti_tx_hiz", &self.tsti_tx_hiz())
            .field("utmo_dig_tst0", &self.utmo_dig_tst0())
            .field("utmo_dig_tst1", &self.utmo_dig_tst1())
            .field("tsti_hsfs_mode_en", &self.tsti_hsfs_mode_en())
            .field("tstpkt", &self.tstpkt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1Loopback {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1Loopback {{ utmi_teststart: {=bool:?}, utmi_dig_tst0: {=bool:?}, utmi_dig_tst1: {=bool:?}, tsti_tx_hs_mode: {=bool:?}, tsti_tx_ls_mode: {=bool:?}, tsti_tx_en: {=bool:?}, tsti_tx_hiz: {=bool:?}, utmo_dig_tst0: {=bool:?}, utmo_dig_tst1: {=bool:?}, tsti_hsfs_mode_en: {=bool:?}, tstpkt: {=u8:?} }}",
            self.utmi_teststart(),
            self.utmi_dig_tst0(),
            self.utmi_dig_tst1(),
            self.tsti_tx_hs_mode(),
            self.tsti_tx_ls_mode(),
            self.tsti_tx_en(),
            self.tsti_tx_hiz(),
            self.utmo_dig_tst0(),
            self.utmo_dig_tst1(),
            self.tsti_hsfs_mode_en(),
            self.tstpkt()
        )
    }
}
#[doc = "USB PHY Loopback Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackClr(pub u32);
impl Usb1LoopbackClr {
    #[doc = "This bit enables the USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the USB loopback test."]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hs_mode(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub const fn set_tsti_tx_hs_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_ls_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub const fn set_tsti_tx_ls_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable TX for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hiz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hsfs_mode_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub const fn set_tsti_hsfs_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tstpkt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub const fn set_tstpkt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Usb1LoopbackClr {
    #[inline(always)]
    fn default() -> Usb1LoopbackClr {
        Usb1LoopbackClr(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackClr")
            .field("utmi_teststart", &self.utmi_teststart())
            .field("utmi_dig_tst0", &self.utmi_dig_tst0())
            .field("utmi_dig_tst1", &self.utmi_dig_tst1())
            .field("tsti_tx_hs_mode", &self.tsti_tx_hs_mode())
            .field("tsti_tx_ls_mode", &self.tsti_tx_ls_mode())
            .field("tsti_tx_en", &self.tsti_tx_en())
            .field("tsti_tx_hiz", &self.tsti_tx_hiz())
            .field("utmo_dig_tst0", &self.utmo_dig_tst0())
            .field("utmo_dig_tst1", &self.utmo_dig_tst1())
            .field("tsti_hsfs_mode_en", &self.tsti_hsfs_mode_en())
            .field("tstpkt", &self.tstpkt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackClr {{ utmi_teststart: {=bool:?}, utmi_dig_tst0: {=bool:?}, utmi_dig_tst1: {=bool:?}, tsti_tx_hs_mode: {=bool:?}, tsti_tx_ls_mode: {=bool:?}, tsti_tx_en: {=bool:?}, tsti_tx_hiz: {=bool:?}, utmo_dig_tst0: {=bool:?}, utmo_dig_tst1: {=bool:?}, tsti_hsfs_mode_en: {=bool:?}, tstpkt: {=u8:?} }}",
            self.utmi_teststart(),
            self.utmi_dig_tst0(),
            self.utmi_dig_tst1(),
            self.tsti_tx_hs_mode(),
            self.tsti_tx_ls_mode(),
            self.tsti_tx_en(),
            self.tsti_tx_hiz(),
            self.utmo_dig_tst0(),
            self.utmo_dig_tst1(),
            self.tsti_hsfs_mode_en(),
            self.tstpkt()
        )
    }
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackHsfscnt(pub u32);
impl Usb1LoopbackHsfscnt {
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hs_number(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_hs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_fs_number(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_fs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Usb1LoopbackHsfscnt {
    #[inline(always)]
    fn default() -> Usb1LoopbackHsfscnt {
        Usb1LoopbackHsfscnt(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackHsfscnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackHsfscnt")
            .field("tsti_hs_number", &self.tsti_hs_number())
            .field("tsti_fs_number", &self.tsti_fs_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackHsfscnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackHsfscnt {{ tsti_hs_number: {=u16:?}, tsti_fs_number: {=u16:?} }}",
            self.tsti_hs_number(),
            self.tsti_fs_number()
        )
    }
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackHsfscntClr(pub u32);
impl Usb1LoopbackHsfscntClr {
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hs_number(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_hs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_fs_number(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_fs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Usb1LoopbackHsfscntClr {
    #[inline(always)]
    fn default() -> Usb1LoopbackHsfscntClr {
        Usb1LoopbackHsfscntClr(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackHsfscntClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackHsfscntClr")
            .field("tsti_hs_number", &self.tsti_hs_number())
            .field("tsti_fs_number", &self.tsti_fs_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackHsfscntClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackHsfscntClr {{ tsti_hs_number: {=u16:?}, tsti_fs_number: {=u16:?} }}",
            self.tsti_hs_number(),
            self.tsti_fs_number()
        )
    }
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackHsfscntSet(pub u32);
impl Usb1LoopbackHsfscntSet {
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hs_number(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_hs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_fs_number(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_fs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Usb1LoopbackHsfscntSet {
    #[inline(always)]
    fn default() -> Usb1LoopbackHsfscntSet {
        Usb1LoopbackHsfscntSet(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackHsfscntSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackHsfscntSet")
            .field("tsti_hs_number", &self.tsti_hs_number())
            .field("tsti_fs_number", &self.tsti_fs_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackHsfscntSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackHsfscntSet {{ tsti_hs_number: {=u16:?}, tsti_fs_number: {=u16:?} }}",
            self.tsti_hs_number(),
            self.tsti_fs_number()
        )
    }
}
#[doc = "USB PHY Loopback Packet Number Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackHsfscntTog(pub u32);
impl Usb1LoopbackHsfscntTog {
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hs_number(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "High speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_hs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_fs_number(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Full speed packet number, used when USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_tsti_fs_number(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Usb1LoopbackHsfscntTog {
    #[inline(always)]
    fn default() -> Usb1LoopbackHsfscntTog {
        Usb1LoopbackHsfscntTog(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackHsfscntTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackHsfscntTog")
            .field("tsti_hs_number", &self.tsti_hs_number())
            .field("tsti_fs_number", &self.tsti_fs_number())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackHsfscntTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackHsfscntTog {{ tsti_hs_number: {=u16:?}, tsti_fs_number: {=u16:?} }}",
            self.tsti_hs_number(),
            self.tsti_fs_number()
        )
    }
}
#[doc = "USB PHY Loopback Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackSet(pub u32);
impl Usb1LoopbackSet {
    #[doc = "This bit enables the USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the USB loopback test."]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hs_mode(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub const fn set_tsti_tx_hs_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_ls_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub const fn set_tsti_tx_ls_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable TX for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hiz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hsfs_mode_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub const fn set_tsti_hsfs_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tstpkt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub const fn set_tstpkt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Usb1LoopbackSet {
    #[inline(always)]
    fn default() -> Usb1LoopbackSet {
        Usb1LoopbackSet(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackSet")
            .field("utmi_teststart", &self.utmi_teststart())
            .field("utmi_dig_tst0", &self.utmi_dig_tst0())
            .field("utmi_dig_tst1", &self.utmi_dig_tst1())
            .field("tsti_tx_hs_mode", &self.tsti_tx_hs_mode())
            .field("tsti_tx_ls_mode", &self.tsti_tx_ls_mode())
            .field("tsti_tx_en", &self.tsti_tx_en())
            .field("tsti_tx_hiz", &self.tsti_tx_hiz())
            .field("utmo_dig_tst0", &self.utmo_dig_tst0())
            .field("utmo_dig_tst1", &self.utmo_dig_tst1())
            .field("tsti_hsfs_mode_en", &self.tsti_hsfs_mode_en())
            .field("tstpkt", &self.tstpkt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackSet {{ utmi_teststart: {=bool:?}, utmi_dig_tst0: {=bool:?}, utmi_dig_tst1: {=bool:?}, tsti_tx_hs_mode: {=bool:?}, tsti_tx_ls_mode: {=bool:?}, tsti_tx_en: {=bool:?}, tsti_tx_hiz: {=bool:?}, utmo_dig_tst0: {=bool:?}, utmo_dig_tst1: {=bool:?}, tsti_hsfs_mode_en: {=bool:?}, tstpkt: {=u8:?} }}",
            self.utmi_teststart(),
            self.utmi_dig_tst0(),
            self.utmi_dig_tst1(),
            self.tsti_tx_hs_mode(),
            self.tsti_tx_ls_mode(),
            self.tsti_tx_en(),
            self.tsti_tx_hiz(),
            self.utmo_dig_tst0(),
            self.utmo_dig_tst1(),
            self.tsti_hsfs_mode_en(),
            self.tstpkt()
        )
    }
}
#[doc = "USB PHY Loopback Control/Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1LoopbackTog(pub u32);
impl Usb1LoopbackTog {
    #[doc = "This bit enables the USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_teststart(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the USB loopback test."]
    #[inline(always)]
    pub const fn set_utmi_teststart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mode control for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_dig_tst1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mode control for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmi_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hs_mode(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Select HS or FS mode for USB loopback testing"]
    #[inline(always)]
    pub const fn set_tsti_tx_hs_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_ls_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline(always)]
    pub const fn set_tsti_tx_ls_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable TX for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable TX for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_tx_hiz(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Sets TX Hi-Z for USB loopback test."]
    #[inline(always)]
    pub const fn set_tsti_tx_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test results"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[must_use]
    #[inline(always)]
    pub const fn utmo_dig_tst1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit is a status bit for USB loopback test"]
    #[inline(always)]
    pub const fn set_utmo_dig_tst1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[must_use]
    #[inline(always)]
    pub const fn tsti_hsfs_mode_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline(always)]
    pub const fn set_tsti_hsfs_mode_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tstpkt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline(always)]
    pub const fn set_tstpkt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Usb1LoopbackTog {
    #[inline(always)]
    fn default() -> Usb1LoopbackTog {
        Usb1LoopbackTog(0)
    }
}
impl core::fmt::Debug for Usb1LoopbackTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1LoopbackTog")
            .field("utmi_teststart", &self.utmi_teststart())
            .field("utmi_dig_tst0", &self.utmi_dig_tst0())
            .field("utmi_dig_tst1", &self.utmi_dig_tst1())
            .field("tsti_tx_hs_mode", &self.tsti_tx_hs_mode())
            .field("tsti_tx_ls_mode", &self.tsti_tx_ls_mode())
            .field("tsti_tx_en", &self.tsti_tx_en())
            .field("tsti_tx_hiz", &self.tsti_tx_hiz())
            .field("utmo_dig_tst0", &self.utmo_dig_tst0())
            .field("utmo_dig_tst1", &self.utmo_dig_tst1())
            .field("tsti_hsfs_mode_en", &self.tsti_hsfs_mode_en())
            .field("tstpkt", &self.tstpkt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1LoopbackTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1LoopbackTog {{ utmi_teststart: {=bool:?}, utmi_dig_tst0: {=bool:?}, utmi_dig_tst1: {=bool:?}, tsti_tx_hs_mode: {=bool:?}, tsti_tx_ls_mode: {=bool:?}, tsti_tx_en: {=bool:?}, tsti_tx_hiz: {=bool:?}, utmo_dig_tst0: {=bool:?}, utmo_dig_tst1: {=bool:?}, tsti_hsfs_mode_en: {=bool:?}, tstpkt: {=u8:?} }}",
            self.utmi_teststart(),
            self.utmi_dig_tst0(),
            self.utmi_dig_tst1(),
            self.tsti_tx_hs_mode(),
            self.tsti_tx_ls_mode(),
            self.tsti_tx_en(),
            self.tsti_tx_hiz(),
            self.utmo_dig_tst0(),
            self.utmo_dig_tst1(),
            self.tsti_hsfs_mode_en(),
            self.tstpkt()
        )
    }
}
#[doc = "USB PHY VBUS Detector Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStat(pub u32);
impl Usb1VbusDetStat {
    #[doc = "Session End indicator Session End status, value inverted from Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> super::vals::Sessend {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sessend::from_bits(val as u8)
    }
    #[doc = "Session End indicator Session End status, value inverted from Session Valid comparator"]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: super::vals::Sessend) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> super::vals::Bvalid {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bvalid::from_bits(val as u8)
    }
    #[doc = "B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: super::vals::Bvalid) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> super::vals::Avalid {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Avalid::from_bits(val as u8)
    }
    #[doc = "A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: super::vals::Avalid) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> super::vals::VbusValid {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VbusValid::from_bits(val as u8)
    }
    #[doc = "VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin"]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: super::vals::VbusValid) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> super::vals::VbusValid3v {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VbusValid3v::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators"]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: super::vals::VbusValid3v) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStat {{ sessend: {:?}, bvalid: {:?}, avalid: {:?}, vbus_valid: {:?}, vbus_valid_3v: {:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetect(pub u32);
impl Usb1VbusDetect {
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: super::vals::Usb1VbusDetectVbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> super::vals::Usb1VbusDetectVbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usb1VbusDetectVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: super::vals::Usb1VbusDetectVbusOverrideEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> super::vals::Usb1VbusDetectVbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Usb1VbusDetectVbusvalidSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: super::vals::Usb1VbusDetectVbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::Usb1VbusDetectVbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Usb1VbusDetectVbusSourceSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::Usb1VbusDetectVbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_sessvalid(&self) -> super::vals::Usb1VbusDetectVbusvalidToSessvalid {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usb1VbusDetectVbusvalidToSessvalid::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_sessvalid(
        &mut self,
        val: super::vals::Usb1VbusDetectVbusvalidToSessvalid,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectPwrupCmps {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1VbusDetectPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectPwrupCmps) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> super::vals::Usb1VbusDetectDischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Usb1VbusDetectDischargeVbus::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: super::vals::Usb1VbusDetectDischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[must_use]
    #[inline(always)]
    pub const fn en_charger_resistor(&self) -> super::vals::Usb1VbusDetectEnChargerResistor {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb1VbusDetectEnChargerResistor::from_bits(val as u8)
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub const fn set_en_charger_resistor(
        &mut self,
        val: super::vals::Usb1VbusDetectEnChargerResistor,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("en_charger_resistor", &self.en_charger_resistor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetect {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, vbusvalid_to_sessvalid: {:?}, pwrup_cmps: {:?}, discharge_vbus: {:?}, en_charger_resistor: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.vbusvalid_to_sessvalid(),
            self.pwrup_cmps(),
            self.discharge_vbus(),
            self.en_charger_resistor()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectClr(pub u32);
impl Usb1VbusDetectClr {
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectClrVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectClrVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectClrVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> super::vals::Usb1VbusDetectClrVbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usb1VbusDetectClrVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectClrVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> super::vals::Usb1VbusDetectClrVbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Usb1VbusDetectClrVbusvalidSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: super::vals::Usb1VbusDetectClrVbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::Usb1VbusDetectClrVbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Usb1VbusDetectClrVbusSourceSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::Usb1VbusDetectClrVbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_sessvalid(
        &self,
    ) -> super::vals::Usb1VbusDetectClrVbusvalidToSessvalid {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usb1VbusDetectClrVbusvalidToSessvalid::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_sessvalid(
        &mut self,
        val: super::vals::Usb1VbusDetectClrVbusvalidToSessvalid,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectClrPwrupCmps {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1VbusDetectClrPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectClrPwrupCmps) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> super::vals::Usb1VbusDetectClrDischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Usb1VbusDetectClrDischargeVbus::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: super::vals::Usb1VbusDetectClrDischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[must_use]
    #[inline(always)]
    pub const fn en_charger_resistor(&self) -> super::vals::Usb1VbusDetectClrEnChargerResistor {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb1VbusDetectClrEnChargerResistor::from_bits(val as u8)
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub const fn set_en_charger_resistor(
        &mut self,
        val: super::vals::Usb1VbusDetectClrEnChargerResistor,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("en_charger_resistor", &self.en_charger_resistor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectClr {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, vbusvalid_to_sessvalid: {:?}, pwrup_cmps: {:?}, discharge_vbus: {:?}, en_charger_resistor: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.vbusvalid_to_sessvalid(),
            self.pwrup_cmps(),
            self.discharge_vbus(),
            self.en_charger_resistor()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectSet(pub u32);
impl Usb1VbusDetectSet {
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectSetVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectSetVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectSetVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> super::vals::Usb1VbusDetectSetVbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usb1VbusDetectSetVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectSetVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> super::vals::Usb1VbusDetectSetVbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Usb1VbusDetectSetVbusvalidSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: super::vals::Usb1VbusDetectSetVbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::Usb1VbusDetectSetVbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Usb1VbusDetectSetVbusSourceSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::Usb1VbusDetectSetVbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_sessvalid(
        &self,
    ) -> super::vals::Usb1VbusDetectSetVbusvalidToSessvalid {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usb1VbusDetectSetVbusvalidToSessvalid::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_sessvalid(
        &mut self,
        val: super::vals::Usb1VbusDetectSetVbusvalidToSessvalid,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectSetPwrupCmps {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1VbusDetectSetPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectSetPwrupCmps) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> super::vals::Usb1VbusDetectSetDischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Usb1VbusDetectSetDischargeVbus::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: super::vals::Usb1VbusDetectSetDischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[must_use]
    #[inline(always)]
    pub const fn en_charger_resistor(&self) -> super::vals::Usb1VbusDetectSetEnChargerResistor {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb1VbusDetectSetEnChargerResistor::from_bits(val as u8)
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub const fn set_en_charger_resistor(
        &mut self,
        val: super::vals::Usb1VbusDetectSetEnChargerResistor,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("en_charger_resistor", &self.en_charger_resistor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectSet {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, vbusvalid_to_sessvalid: {:?}, pwrup_cmps: {:?}, discharge_vbus: {:?}, en_charger_resistor: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.vbusvalid_to_sessvalid(),
            self.pwrup_cmps(),
            self.discharge_vbus(),
            self.en_charger_resistor()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectTog(pub u32);
impl Usb1VbusDetectTog {
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> super::vals::Usb1VbusDetectTogVbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Usb1VbusDetectTogVbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(
        &mut self,
        val: super::vals::Usb1VbusDetectTogVbusvalidThresh,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> super::vals::Usb1VbusDetectTogVbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usb1VbusDetectTogVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable"]
    #[inline(always)]
    pub const fn set_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectTogVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1"]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1"]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> super::vals::Usb1VbusDetectTogVbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Usb1VbusDetectTogVbusvalidSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: super::vals::Usb1VbusDetectTogVbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::Usb1VbusDetectTogVbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Usb1VbusDetectTogVbusSourceSel::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::Usb1VbusDetectTogVbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_sessvalid(
        &self,
    ) -> super::vals::Usb1VbusDetectTogVbusvalidToSessvalid {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usb1VbusDetectTogVbusvalidToSessvalid::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator"]
    #[inline(always)]
    pub const fn set_vbusvalid_to_sessvalid(
        &mut self,
        val: super::vals::Usb1VbusDetectTogVbusvalidToSessvalid,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectTogPwrupCmps {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Usb1VbusDetectTogPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectTogPwrupCmps) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> super::vals::Usb1VbusDetectTogDischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Usb1VbusDetectTogDischargeVbus::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground"]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: super::vals::Usb1VbusDetectTogDischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[must_use]
    #[inline(always)]
    pub const fn en_charger_resistor(&self) -> super::vals::Usb1VbusDetectTogEnChargerResistor {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Usb1VbusDetectTogEnChargerResistor::from_bits(val as u8)
    }
    #[doc = "Enables resistors used for an older method of resistive battery charger detection"]
    #[inline(always)]
    pub const fn set_en_charger_resistor(
        &mut self,
        val: super::vals::Usb1VbusDetectTogEnChargerResistor,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .field("en_charger_resistor", &self.en_charger_resistor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectTog {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, vbusvalid_to_sessvalid: {:?}, pwrup_cmps: {:?}, discharge_vbus: {:?}, en_charger_resistor: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.vbusvalid_to_sessvalid(),
            self.pwrup_cmps(),
            self.discharge_vbus(),
            self.en_charger_resistor()
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
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MINOR field of the RTL version"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL versio"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Fixed read-only value reflecting the MAJOR field of the RTL versio"]
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
