#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl(pub u32);
impl Anactrl {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
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
            "Anactrl {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlClr(pub u32);
impl AnactrlClr {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
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
            "AnactrlClr {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlSet(pub u32);
impl AnactrlSet {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
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
            "AnactrlSet {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "USB PHY Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlTog(pub u32);
impl AnactrlTog {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
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
            "AnactrlTog {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
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
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_usbclkgate(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_usbclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoset_usbclks(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoset_usbclks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("enautoclr_usbclkgate", &self.enautoclr_usbclkgate())
            .field("enautoset_usbclks", &self.enautoset_usbclks())
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
            "Ctrl {{ enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_polarity: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, enautoclr_usbclkgate: {=bool:?}, enautoset_usbclks: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_polarity(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.envbuschg_wkup(),
            self.enautoclr_usbclkgate(),
            self.enautoset_usbclks(),
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
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_usbclkgate(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_usbclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoset_usbclks(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoset_usbclks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("enautoclr_usbclkgate", &self.enautoclr_usbclkgate())
            .field("enautoset_usbclks", &self.enautoset_usbclks())
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
            "CtrlClr {{ enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_polarity: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, enautoclr_usbclkgate: {=bool:?}, enautoset_usbclks: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_polarity(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.envbuschg_wkup(),
            self.enautoclr_usbclkgate(),
            self.enautoset_usbclks(),
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
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_usbclkgate(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_usbclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoset_usbclks(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoset_usbclks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("enautoclr_usbclkgate", &self.enautoclr_usbclkgate())
            .field("enautoset_usbclks", &self.enautoset_usbclks())
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
            "CtrlSet {{ enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_polarity: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, enautoclr_usbclkgate: {=bool:?}, enautoset_usbclks: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_polarity(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.envbuschg_wkup(),
            self.enautoclr_usbclkgate(),
            self.enautoset_usbclks(),
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
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[must_use]
    #[inline(always)]
    pub const fn endpdmchg_wkup(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub const fn set_endpdmchg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn envbuschg_wkup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub const fn set_envbuschg_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_usbclkgate(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoclr_usbclkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn enautoset_usbclks(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub const fn set_enautoset_usbclks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindet", &self.endevplugindet())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("endpdmchg_wkup", &self.endpdmchg_wkup())
            .field("envbuschg_wkup", &self.envbuschg_wkup())
            .field("enautoclr_usbclkgate", &self.enautoclr_usbclkgate())
            .field("enautoset_usbclks", &self.enautoset_usbclks())
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
            "CtrlTog {{ enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindet: {:?}, devplugin_polarity: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, devplugin_irq: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, endpdmchg_wkup: {=bool:?}, envbuschg_wkup: {=bool:?}, enautoclr_usbclkgate: {=bool:?}, enautoset_usbclks: {=bool:?}, host_force_ls_se0: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindet(),
            self.devplugin_polarity(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.devplugin_irq(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.endpdmchg_wkup(),
            self.envbuschg_wkup(),
            self.enautoclr_usbclkgate(),
            self.enautoset_usbclks(),
            self.host_force_ls_se0(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
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
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_prediv(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_prediv", &self.pll_prediv())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSic {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSic {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_prediv: {=bool:?}, pll_lock: {:?} }}",
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_prediv(),
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
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_prediv(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_prediv", &self.pll_prediv())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicClr {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_prediv: {=bool:?}, pll_lock: {:?} }}",
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_prediv(),
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
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_prediv(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_prediv", &self.pll_prediv())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicSet {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_prediv: {=bool:?}, pll_lock: {:?} }}",
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_prediv(),
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
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_prediv(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_pll_prediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_prediv", &self.pll_prediv())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicTog {{ pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, refbias_pwd_sel: {:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {:?}, pll_prediv: {=bool:?}, pll_lock: {:?} }}",
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_prediv(),
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
    #[doc = "Indicates the USB 3v power rails are in range."]
    #[must_use]
    #[inline(always)]
    pub const fn ok_status_3v(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the USB 3v power rails are in range."]
    #[inline(always)]
    pub const fn set_ok_status_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
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
            .field("ok_status_3v", &self.ok_status_3v())
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
            "Status {{ ok_status_3v: {=bool:?}, hostdiscondetect_status: {:?}, devplugin_status: {:?}, resume_status: {=bool:?} }}",
            self.ok_status_3v(),
            self.hostdiscondetect_status(),
            self.devplugin_status(),
            self.resume_status()
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
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> super::vals::Usb1VbusDetectExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Usb1VbusDetectExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectExtIdOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> super::vals::Usb1VbusDetectExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usb1VbusDetectExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectExtVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_5vdetect(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbusvalid_5vdetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Usb1VbusDetectPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("vbusvalid_5vdetect", &self.vbusvalid_5vdetect())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetect {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_sessvalid: {:?}, vbusvalid_5vdetect: {=bool:?}, pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
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
            self.vbusvalid_to_sessvalid(),
            self.vbusvalid_5vdetect(),
            self.pwrup_cmps(),
            self.discharge_vbus()
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
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> super::vals::Usb1VbusDetectClrExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Usb1VbusDetectClrExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectClrExtIdOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> super::vals::Usb1VbusDetectClrExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usb1VbusDetectClrExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectClrExtVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_5vdetect(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbusvalid_5vdetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectClrPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Usb1VbusDetectClrPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectClrPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("vbusvalid_5vdetect", &self.vbusvalid_5vdetect())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectClr {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_sessvalid: {:?}, vbusvalid_5vdetect: {=bool:?}, pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
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
            self.vbusvalid_to_sessvalid(),
            self.vbusvalid_5vdetect(),
            self.pwrup_cmps(),
            self.discharge_vbus()
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
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> super::vals::Usb1VbusDetectSetExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Usb1VbusDetectSetExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:"]
    #[inline(always)]
    pub const fn set_ext_id_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectSetExtIdOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> super::vals::Usb1VbusDetectSetExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usb1VbusDetectSetExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectSetExtVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_5vdetect(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbusvalid_5vdetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectSetPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Usb1VbusDetectSetPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectSetPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("vbusvalid_5vdetect", &self.vbusvalid_5vdetect())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectSet {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_sessvalid: {:?}, vbusvalid_5vdetect: {=bool:?}, pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
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
            self.vbusvalid_to_sessvalid(),
            self.vbusvalid_5vdetect(),
            self.pwrup_cmps(),
            self.discharge_vbus()
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
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> super::vals::Usb1VbusDetectTogExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Usb1VbusDetectTogExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_ext_id_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectTogExtIdOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> super::vals::Usb1VbusDetectTogExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Usb1VbusDetectTogExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(
        &mut self,
        val: super::vals::Usb1VbusDetectTogExtVbusOverrideEn,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_5vdetect(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vbusvalid_5vdetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrup_cmps(&self) -> super::vals::Usb1VbusDetectTogPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Usb1VbusDetectTogPwrupCmps::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector"]
    #[inline(always)]
    pub const fn set_pwrup_cmps(&mut self, val: super::vals::Usb1VbusDetectTogPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
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
            .field("vbusvalid_to_sessvalid", &self.vbusvalid_to_sessvalid())
            .field("vbusvalid_5vdetect", &self.vbusvalid_5vdetect())
            .field("pwrup_cmps", &self.pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectTog {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_sessvalid: {:?}, vbusvalid_5vdetect: {=bool:?}, pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
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
            self.vbusvalid_to_sessvalid(),
            self.vbusvalid_5vdetect(),
            self.pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
