#[doc = "USB PHY Analog Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL(pub u32);
impl ANACTRL {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> super::vals::ANACTRL_DEV_PULLDOWN {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ANACTRL_DEV_PULLDOWN::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: super::vals::ANACTRL_DEV_PULLDOWN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL {
    #[inline(always)]
    fn default() -> ANACTRL {
        ANACTRL(0)
    }
}
impl core::fmt::Debug for ANACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "USB PHY Analog Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_CLR(pub u32);
impl ANACTRL_CLR {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> super::vals::ANACTRL_CLR_DEV_PULLDOWN {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ANACTRL_CLR_DEV_PULLDOWN::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: super::vals::ANACTRL_CLR_DEV_PULLDOWN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_CLR {
    #[inline(always)]
    fn default() -> ANACTRL_CLR {
        ANACTRL_CLR(0)
    }
}
impl core::fmt::Debug for ANACTRL_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_CLR")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_CLR {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "USB PHY Analog Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_SET(pub u32);
impl ANACTRL_SET {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> super::vals::ANACTRL_SET_DEV_PULLDOWN {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ANACTRL_SET_DEV_PULLDOWN::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: super::vals::ANACTRL_SET_DEV_PULLDOWN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_SET {
    #[inline(always)]
    fn default() -> ANACTRL_SET {
        ANACTRL_SET(0)
    }
}
impl core::fmt::Debug for ANACTRL_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_SET")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_SET {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "USB PHY Analog Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_TOG(pub u32);
impl ANACTRL_TOG {
    #[doc = "Vow voltage detector enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Vow voltage detector enable bit."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> super::vals::ANACTRL_TOG_DEV_PULLDOWN {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::ANACTRL_TOG_DEV_PULLDOWN::from_bits(val as u8)
    }
    #[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: super::vals::ANACTRL_TOG_DEV_PULLDOWN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_TOG {
    #[inline(always)]
    fn default() -> ANACTRL_TOG {
        ANACTRL_TOG(0)
    }
}
impl core::fmt::Debug for ANACTRL_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_TOG")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_TOG {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "USB PHY General Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDET(&self) -> super::vals::CTRL_ENDEVPLUGINDET {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CTRL_ENDEVPLUGINDET::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDET(&mut self, val: super::vals::CTRL_ENDEVPLUGINDET) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the device is connected."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDPDMCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[inline(always)]
    pub const fn set_ENDPDMCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVBUSCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_ENVBUSCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_USBCLKGATE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_USBCLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOSET_USBCLKS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOSET_USBCLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_FORCE_LS_SE0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[inline(always)]
    pub const fn set_HOST_FORCE_LS_SE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDET", &self.ENDEVPLUGINDET())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("ENDPDMCHG_WKUP", &self.ENDPDMCHG_WKUP())
            .field("ENVBUSCHG_WKUP", &self.ENVBUSCHG_WKUP())
            .field("ENAUTOCLR_USBCLKGATE", &self.ENAUTOCLR_USBCLKGATE())
            .field("ENAUTOSET_USBCLKS", &self.ENAUTOSET_USBCLKS())
            .field("HOST_FORCE_LS_SE0", &self.HOST_FORCE_LS_SE0())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDET: {:?}, DEVPLUGIN_POLARITY: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, ENDPDMCHG_WKUP: {=bool:?}, ENVBUSCHG_WKUP: {=bool:?}, ENAUTOCLR_USBCLKGATE: {=bool:?}, ENAUTOSET_USBCLKS: {=bool:?}, HOST_FORCE_LS_SE0: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDET(),
            self.DEVPLUGIN_POLARITY(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.DEVPLUGIN_IRQ(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.ENDPDMCHG_WKUP(),
            self.ENVBUSCHG_WKUP(),
            self.ENAUTOCLR_USBCLKGATE(),
            self.ENAUTOSET_USBCLKS(),
            self.HOST_FORCE_LS_SE0(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "USB PHY General Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_CLR(pub u32);
impl CTRL_CLR {
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDET(&self) -> super::vals::CTRL_CLR_ENDEVPLUGINDET {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CTRL_CLR_ENDEVPLUGINDET::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDET(&mut self, val: super::vals::CTRL_CLR_ENDEVPLUGINDET) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the device is connected."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDPDMCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[inline(always)]
    pub const fn set_ENDPDMCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVBUSCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_ENVBUSCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_USBCLKGATE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_USBCLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOSET_USBCLKS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOSET_USBCLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_FORCE_LS_SE0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[inline(always)]
    pub const fn set_HOST_FORCE_LS_SE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_CLR {
    #[inline(always)]
    fn default() -> CTRL_CLR {
        CTRL_CLR(0)
    }
}
impl core::fmt::Debug for CTRL_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CLR")
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDET", &self.ENDEVPLUGINDET())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("ENDPDMCHG_WKUP", &self.ENDPDMCHG_WKUP())
            .field("ENVBUSCHG_WKUP", &self.ENVBUSCHG_WKUP())
            .field("ENAUTOCLR_USBCLKGATE", &self.ENAUTOCLR_USBCLKGATE())
            .field("ENAUTOSET_USBCLKS", &self.ENAUTOSET_USBCLKS())
            .field("HOST_FORCE_LS_SE0", &self.HOST_FORCE_LS_SE0())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_CLR {{ ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDET: {:?}, DEVPLUGIN_POLARITY: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, ENDPDMCHG_WKUP: {=bool:?}, ENVBUSCHG_WKUP: {=bool:?}, ENAUTOCLR_USBCLKGATE: {=bool:?}, ENAUTOSET_USBCLKS: {=bool:?}, HOST_FORCE_LS_SE0: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDET(),
            self.DEVPLUGIN_POLARITY(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.DEVPLUGIN_IRQ(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.ENDPDMCHG_WKUP(),
            self.ENVBUSCHG_WKUP(),
            self.ENAUTOCLR_USBCLKGATE(),
            self.ENAUTOSET_USBCLKS(),
            self.HOST_FORCE_LS_SE0(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "USB PHY General Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_SET(pub u32);
impl CTRL_SET {
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDET(&self) -> super::vals::CTRL_SET_ENDEVPLUGINDET {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CTRL_SET_ENDEVPLUGINDET::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDET(&mut self, val: super::vals::CTRL_SET_ENDEVPLUGINDET) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the device is connected."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDPDMCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[inline(always)]
    pub const fn set_ENDPDMCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVBUSCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_ENVBUSCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_USBCLKGATE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_USBCLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOSET_USBCLKS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOSET_USBCLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_FORCE_LS_SE0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[inline(always)]
    pub const fn set_HOST_FORCE_LS_SE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_SET {
    #[inline(always)]
    fn default() -> CTRL_SET {
        CTRL_SET(0)
    }
}
impl core::fmt::Debug for CTRL_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_SET")
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDET", &self.ENDEVPLUGINDET())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("ENDPDMCHG_WKUP", &self.ENDPDMCHG_WKUP())
            .field("ENVBUSCHG_WKUP", &self.ENVBUSCHG_WKUP())
            .field("ENAUTOCLR_USBCLKGATE", &self.ENAUTOCLR_USBCLKGATE())
            .field("ENAUTOSET_USBCLKS", &self.ENAUTOSET_USBCLKS())
            .field("HOST_FORCE_LS_SE0", &self.HOST_FORCE_LS_SE0())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_SET {{ ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDET: {:?}, DEVPLUGIN_POLARITY: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, ENDPDMCHG_WKUP: {=bool:?}, ENVBUSCHG_WKUP: {=bool:?}, ENAUTOCLR_USBCLKGATE: {=bool:?}, ENAUTOSET_USBCLKS: {=bool:?}, HOST_FORCE_LS_SE0: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDET(),
            self.DEVPLUGIN_POLARITY(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.DEVPLUGIN_IRQ(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.ENDPDMCHG_WKUP(),
            self.ENVBUSCHG_WKUP(),
            self.ENAUTOCLR_USBCLKGATE(),
            self.ENAUTOSET_USBCLKS(),
            self.HOST_FORCE_LS_SE0(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "USB PHY General Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_TOG(pub u32);
impl CTRL_TOG {
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "For host mode, enables high-speed disconnect detector."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device has disconnected in High-Speed mode."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDET(&self) -> super::vals::CTRL_TOG_ENDEVPLUGINDET {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CTRL_TOG_ENDEVPLUGINDET::from_bits(val as u8)
    }
    #[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDET(&mut self, val: super::vals::CTRL_TOG_ENDEVPLUGINDET) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume IRQ: Indicates that the host is sending a wake-up after suspend."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the device is connected."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the device is connected."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 2 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables UTMI+ Level 3 operation for the USB HS PHY."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up IRQ: Indicates that there is a wak-eup event."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDPDMCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DP DM change wake-up: Not for customer use."]
    #[inline(always)]
    pub const fn set_ENDPDMCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVBUSCHG_WKUP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub const fn set_ENVBUSCHG_WKUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_USBCLKGATE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_USBCLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOSET_USBCLKS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended."]
    #[inline(always)]
    pub const fn set_ENAUTOSET_USBCLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[must_use]
    #[inline(always)]
    pub const fn HOST_FORCE_LS_SE0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the next FS packet that is transmitted to have a EOP with low-speed timing."]
    #[inline(always)]
    pub const fn set_HOST_FORCE_LS_SE0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Used by the PHY to indicate a powered-down state."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Gate UTMI Clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Gate UTMI Clocks."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_TOG {
    #[inline(always)]
    fn default() -> CTRL_TOG {
        CTRL_TOG(0)
    }
}
impl core::fmt::Debug for CTRL_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_TOG")
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDET", &self.ENDEVPLUGINDET())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("ENDPDMCHG_WKUP", &self.ENDPDMCHG_WKUP())
            .field("ENVBUSCHG_WKUP", &self.ENVBUSCHG_WKUP())
            .field("ENAUTOCLR_USBCLKGATE", &self.ENAUTOCLR_USBCLKGATE())
            .field("ENAUTOSET_USBCLKS", &self.ENAUTOSET_USBCLKS())
            .field("HOST_FORCE_LS_SE0", &self.HOST_FORCE_LS_SE0())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_TOG {{ ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDET: {:?}, DEVPLUGIN_POLARITY: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, ENDPDMCHG_WKUP: {=bool:?}, ENVBUSCHG_WKUP: {=bool:?}, ENAUTOCLR_USBCLKGATE: {=bool:?}, ENAUTOSET_USBCLKS: {=bool:?}, HOST_FORCE_LS_SE0: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDET(),
            self.DEVPLUGIN_POLARITY(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.DEVPLUGIN_IRQ(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.ENDPDMCHG_WKUP(),
            self.ENVBUSCHG_WKUP(),
            self.ENAUTOCLR_USBCLKGATE(),
            self.ENAUTOSET_USBCLKS(),
            self.HOST_FORCE_LS_SE0(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "USB PHY PLL Control/Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC(pub u32);
impl PLL_SIC {
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> super::vals::PLL_SIC_REFBIAS_PWD_SEL {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL_SIC_REFBIAS_PWD_SEL::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: super::vals::PLL_SIC_REFBIAS_PWD_SEL) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> super::vals::PLL_SIC_PLL_DIV_SEL {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PLL_SIC_PLL_DIV_SEL::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: super::vals::PLL_SIC_PLL_DIV_SEL) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL lock status indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> super::vals::PLL_SIC_PLL_LOCK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PLL_SIC_PLL_LOCK::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: super::vals::PLL_SIC_PLL_LOCK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC {
    #[inline(always)]
    fn default() -> PLL_SIC {
        PLL_SIC(0)
    }
}
impl core::fmt::Debug for PLL_SIC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC")
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC {{ PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, REFBIAS_PWD_SEL: {:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {:?} }}",
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "USB PHY PLL Control/Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_CLR(pub u32);
impl PLL_SIC_CLR {
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> super::vals::PLL_SIC_CLR_REFBIAS_PWD_SEL {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL_SIC_CLR_REFBIAS_PWD_SEL::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: super::vals::PLL_SIC_CLR_REFBIAS_PWD_SEL) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> super::vals::PLL_SIC_CLR_PLL_DIV_SEL {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PLL_SIC_CLR_PLL_DIV_SEL::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: super::vals::PLL_SIC_CLR_PLL_DIV_SEL) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL lock status indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> super::vals::PLL_SIC_CLR_PLL_LOCK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PLL_SIC_CLR_PLL_LOCK::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: super::vals::PLL_SIC_CLR_PLL_LOCK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_CLR {
    #[inline(always)]
    fn default() -> PLL_SIC_CLR {
        PLL_SIC_CLR(0)
    }
}
impl core::fmt::Debug for PLL_SIC_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_CLR")
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_CLR {{ PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, REFBIAS_PWD_SEL: {:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {:?} }}",
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "USB PHY PLL Control/Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_SET(pub u32);
impl PLL_SIC_SET {
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> super::vals::PLL_SIC_SET_REFBIAS_PWD_SEL {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL_SIC_SET_REFBIAS_PWD_SEL::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: super::vals::PLL_SIC_SET_REFBIAS_PWD_SEL) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> super::vals::PLL_SIC_SET_PLL_DIV_SEL {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PLL_SIC_SET_PLL_DIV_SEL::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: super::vals::PLL_SIC_SET_PLL_DIV_SEL) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL lock status indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> super::vals::PLL_SIC_SET_PLL_LOCK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PLL_SIC_SET_PLL_LOCK::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: super::vals::PLL_SIC_SET_PLL_LOCK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_SET {
    #[inline(always)]
    fn default() -> PLL_SIC_SET {
        PLL_SIC_SET(0)
    }
}
impl core::fmt::Debug for PLL_SIC_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_SET")
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_SET {{ PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, REFBIAS_PWD_SEL: {:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {:?} }}",
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "USB PHY PLL Control/Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_TOG(pub u32);
impl PLL_SIC_TOG {
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the USB clock from PLL to USB PHY."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Power up the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power up the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock output from the USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reference bias power down select."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> super::vals::PLL_SIC_TOG_REFBIAS_PWD_SEL {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PLL_SIC_TOG_REFBIAS_PWD_SEL::from_bits(val as u8)
    }
    #[doc = "Reference bias power down select."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: super::vals::PLL_SIC_TOG_REFBIAS_PWD_SEL) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This field controls the USB PLL regulator, set to enable the regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> super::vals::PLL_SIC_TOG_PLL_DIV_SEL {
        let val = (self.0 >> 22usize) & 0x07;
        super::vals::PLL_SIC_TOG_PLL_DIV_SEL::from_bits(val as u8)
    }
    #[doc = "This field controls the USB PLL feedback loop divider."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: super::vals::PLL_SIC_TOG_PLL_DIV_SEL) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL lock status indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> super::vals::PLL_SIC_TOG_PLL_LOCK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PLL_SIC_TOG_PLL_LOCK::from_bits(val as u8)
    }
    #[doc = "USB PLL lock status indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: super::vals::PLL_SIC_TOG_PLL_LOCK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_TOG {
    #[inline(always)]
    fn default() -> PLL_SIC_TOG {
        PLL_SIC_TOG(0)
    }
}
impl core::fmt::Debug for PLL_SIC_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_TOG")
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_TOG {{ PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, REFBIAS_PWD_SEL: {:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {:?} }}",
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "USB PHY Power-Down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD(pub u32);
impl PWD {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> super::vals::PWD_TXPWDFS {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PWD_TXPWDFS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: super::vals::PWD_TXPWDFS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> super::vals::PWD_TXPWDIBIAS {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PWD_TXPWDIBIAS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: super::vals::PWD_TXPWDIBIAS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> super::vals::PWD_TXPWDV2I {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PWD_TXPWDV2I::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: super::vals::PWD_TXPWDV2I) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> super::vals::PWD_RXPWDENV {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PWD_RXPWDENV::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: super::vals::PWD_RXPWDENV) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> super::vals::PWD_RXPWD1PT1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PWD_RXPWD1PT1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: super::vals::PWD_RXPWD1PT1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> super::vals::PWD_RXPWDDIFF {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PWD_RXPWDDIFF::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: super::vals::PWD_RXPWDDIFF) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> super::vals::PWD_RXPWDRX {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PWD_RXPWDRX::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: super::vals::PWD_RXPWDRX) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD {
    #[inline(always)]
    fn default() -> PWD {
        PWD(0)
    }
}
impl core::fmt::Debug for PWD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD {{ TXPWDFS: {:?}, TXPWDIBIAS: {:?}, TXPWDV2I: {:?}, RXPWDENV: {:?}, RXPWD1PT1: {:?}, RXPWDDIFF: {:?}, RXPWDRX: {:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "USB PHY Power-Down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CLR(pub u32);
impl PWD_CLR {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> super::vals::PWD_CLR_TXPWDFS {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PWD_CLR_TXPWDFS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: super::vals::PWD_CLR_TXPWDFS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> super::vals::PWD_CLR_TXPWDIBIAS {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PWD_CLR_TXPWDIBIAS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: super::vals::PWD_CLR_TXPWDIBIAS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> super::vals::PWD_CLR_TXPWDV2I {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PWD_CLR_TXPWDV2I::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: super::vals::PWD_CLR_TXPWDV2I) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> super::vals::PWD_CLR_RXPWDENV {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PWD_CLR_RXPWDENV::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: super::vals::PWD_CLR_RXPWDENV) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> super::vals::PWD_CLR_RXPWD1PT1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PWD_CLR_RXPWD1PT1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: super::vals::PWD_CLR_RXPWD1PT1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> super::vals::PWD_CLR_RXPWDDIFF {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PWD_CLR_RXPWDDIFF::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: super::vals::PWD_CLR_RXPWDDIFF) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> super::vals::PWD_CLR_RXPWDRX {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PWD_CLR_RXPWDRX::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: super::vals::PWD_CLR_RXPWDRX) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_CLR {
    #[inline(always)]
    fn default() -> PWD_CLR {
        PWD_CLR(0)
    }
}
impl core::fmt::Debug for PWD_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CLR")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CLR {{ TXPWDFS: {:?}, TXPWDIBIAS: {:?}, TXPWDV2I: {:?}, RXPWDENV: {:?}, RXPWD1PT1: {:?}, RXPWDDIFF: {:?}, RXPWDRX: {:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "USB PHY Power-Down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_SET(pub u32);
impl PWD_SET {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> super::vals::PWD_SET_TXPWDFS {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PWD_SET_TXPWDFS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: super::vals::PWD_SET_TXPWDFS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> super::vals::PWD_SET_TXPWDIBIAS {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PWD_SET_TXPWDIBIAS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: super::vals::PWD_SET_TXPWDIBIAS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> super::vals::PWD_SET_TXPWDV2I {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PWD_SET_TXPWDV2I::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: super::vals::PWD_SET_TXPWDV2I) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> super::vals::PWD_SET_RXPWDENV {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PWD_SET_RXPWDENV::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: super::vals::PWD_SET_RXPWDENV) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> super::vals::PWD_SET_RXPWD1PT1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PWD_SET_RXPWD1PT1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: super::vals::PWD_SET_RXPWD1PT1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> super::vals::PWD_SET_RXPWDDIFF {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PWD_SET_RXPWDDIFF::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: super::vals::PWD_SET_RXPWDDIFF) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> super::vals::PWD_SET_RXPWDRX {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PWD_SET_RXPWDRX::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: super::vals::PWD_SET_RXPWDRX) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_SET {
    #[inline(always)]
    fn default() -> PWD_SET {
        PWD_SET(0)
    }
}
impl core::fmt::Debug for PWD_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_SET")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_SET {{ TXPWDFS: {:?}, TXPWDIBIAS: {:?}, TXPWDV2I: {:?}, RXPWDENV: {:?}, RXPWD1PT1: {:?}, RXPWDDIFF: {:?}, RXPWDRX: {:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "USB PHY Power-Down Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_TOG(pub u32);
impl PWD_TOG {
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> super::vals::PWD_TOG_TXPWDFS {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PWD_TOG_TXPWDFS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: super::vals::PWD_TOG_TXPWDFS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> super::vals::PWD_TOG_TXPWDIBIAS {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PWD_TOG_TXPWDIBIAS::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: super::vals::PWD_TOG_TXPWDIBIAS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> super::vals::PWD_TOG_TXPWDV2I {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PWD_TOG_TXPWDV2I::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: super::vals::PWD_TOG_TXPWDV2I) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> super::vals::PWD_TOG_RXPWDENV {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PWD_TOG_RXPWDENV::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: super::vals::PWD_TOG_RXPWDENV) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> super::vals::PWD_TOG_RXPWD1PT1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::PWD_TOG_RXPWD1PT1::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: super::vals::PWD_TOG_RXPWD1PT1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> super::vals::PWD_TOG_RXPWDDIFF {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::PWD_TOG_RXPWDDIFF::from_bits(val as u8)
    }
    #[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: super::vals::PWD_TOG_RXPWDDIFF) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> super::vals::PWD_TOG_RXPWDRX {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::PWD_TOG_RXPWDRX::from_bits(val as u8)
    }
    #[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: super::vals::PWD_TOG_RXPWDRX) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_TOG {
    #[inline(always)]
    fn default() -> PWD_TOG {
        PWD_TOG(0)
    }
}
impl core::fmt::Debug for PWD_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_TOG")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_TOG {{ TXPWDFS: {:?}, TXPWDIBIAS: {:?}, TXPWDV2I: {:?}, RXPWDENV: {:?}, RXPWD1PT1: {:?}, RXPWDDIFF: {:?}, RXPWDRX: {:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "USB PHY Receiver Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX(pub u32);
impl RX {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> super::vals::RX_ENVADJ {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RX_ENVADJ::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: super::vals::RX_ENVADJ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> super::vals::RX_DISCONADJ {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RX_DISCONADJ::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: super::vals::RX_DISCONADJ) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDBYPASS(&self) -> super::vals::RX_RXDBYPASS {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RX_RXDBYPASS::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[inline(always)]
    pub const fn set_RXDBYPASS(&mut self, val: super::vals::RX_RXDBYPASS) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for RX {
    #[inline(always)]
    fn default() -> RX {
        RX(0)
    }
}
impl core::fmt::Debug for RX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .field("RXDBYPASS", &self.RXDBYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX {{ ENVADJ: {:?}, DISCONADJ: {:?}, RXDBYPASS: {:?} }}",
            self.ENVADJ(),
            self.DISCONADJ(),
            self.RXDBYPASS()
        )
    }
}
#[doc = "USB PHY Receiver Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_CLR(pub u32);
impl RX_CLR {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> super::vals::RX_CLR_ENVADJ {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RX_CLR_ENVADJ::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: super::vals::RX_CLR_ENVADJ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> super::vals::RX_CLR_DISCONADJ {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RX_CLR_DISCONADJ::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: super::vals::RX_CLR_DISCONADJ) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDBYPASS(&self) -> super::vals::RX_CLR_RXDBYPASS {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RX_CLR_RXDBYPASS::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[inline(always)]
    pub const fn set_RXDBYPASS(&mut self, val: super::vals::RX_CLR_RXDBYPASS) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for RX_CLR {
    #[inline(always)]
    fn default() -> RX_CLR {
        RX_CLR(0)
    }
}
impl core::fmt::Debug for RX_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CLR")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .field("RXDBYPASS", &self.RXDBYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_CLR {{ ENVADJ: {:?}, DISCONADJ: {:?}, RXDBYPASS: {:?} }}",
            self.ENVADJ(),
            self.DISCONADJ(),
            self.RXDBYPASS()
        )
    }
}
#[doc = "USB PHY Receiver Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_SET(pub u32);
impl RX_SET {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> super::vals::RX_SET_ENVADJ {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RX_SET_ENVADJ::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: super::vals::RX_SET_ENVADJ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> super::vals::RX_SET_DISCONADJ {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RX_SET_DISCONADJ::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: super::vals::RX_SET_DISCONADJ) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDBYPASS(&self) -> super::vals::RX_SET_RXDBYPASS {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RX_SET_RXDBYPASS::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[inline(always)]
    pub const fn set_RXDBYPASS(&mut self, val: super::vals::RX_SET_RXDBYPASS) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for RX_SET {
    #[inline(always)]
    fn default() -> RX_SET {
        RX_SET(0)
    }
}
impl core::fmt::Debug for RX_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_SET")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .field("RXDBYPASS", &self.RXDBYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_SET {{ ENVADJ: {:?}, DISCONADJ: {:?}, RXDBYPASS: {:?} }}",
            self.ENVADJ(),
            self.DISCONADJ(),
            self.RXDBYPASS()
        )
    }
}
#[doc = "USB PHY Receiver Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_TOG(pub u32);
impl RX_TOG {
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> super::vals::RX_TOG_ENVADJ {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::RX_TOG_ENVADJ::from_bits(val as u8)
    }
    #[doc = "The ENVADJ field adjusts the trip point for the envelope detector."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: super::vals::RX_TOG_ENVADJ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> super::vals::RX_TOG_DISCONADJ {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::RX_TOG_DISCONADJ::from_bits(val as u8)
    }
    #[doc = "The DISCONADJ field adjusts the trip point for the disconnect detector."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: super::vals::RX_TOG_DISCONADJ) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXDBYPASS(&self) -> super::vals::RX_TOG_RXDBYPASS {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::RX_TOG_RXDBYPASS::from_bits(val as u8)
    }
    #[doc = "This test mode is intended for lab use only, replace FS differential receiver with DP single ended receiver."]
    #[inline(always)]
    pub const fn set_RXDBYPASS(&mut self, val: super::vals::RX_TOG_RXDBYPASS) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for RX_TOG {
    #[inline(always)]
    fn default() -> RX_TOG {
        RX_TOG(0)
    }
}
impl core::fmt::Debug for RX_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TOG")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .field("RXDBYPASS", &self.RXDBYPASS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_TOG {{ ENVADJ: {:?}, DISCONADJ: {:?}, RXDBYPASS: {:?} }}",
            self.ENVADJ(),
            self.DISCONADJ(),
            self.RXDBYPASS()
        )
    }
}
#[doc = "USB PHY Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "Indicates the USB 3v power rails are in range."]
    #[must_use]
    #[inline(always)]
    pub const fn OK_STATUS_3V(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the USB 3v power rails are in range."]
    #[inline(always)]
    pub const fn set_OK_STATUS_3V(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_STATUS(&self) -> super::vals::HOSTDISCONDETECT_STATUS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::HOSTDISCONDETECT_STATUS::from_bits(val as u8)
    }
    #[doc = "Indicates at the local host (downstream) port that the remote device has disconnected while in High-Speed mode."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_STATUS(&mut self, val: super::vals::HOSTDISCONDETECT_STATUS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_STATUS(&self) -> super::vals::DEVPLUGIN_STATUS {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DEVPLUGIN_STATUS::from_bits(val as u8)
    }
    #[doc = "Status indicator for non-standard resistive plugged-in detection Indicates that the device has been connected on the USB_DP and USB_DM lines using the nonstandard resistive plugged-in detection method controlled by CTRL\\[4\\]."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_STATUS(&mut self, val: super::vals::DEVPLUGIN_STATUS) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_STATUS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the host is sending a wake-up after Suspend and has triggered an interrupt."]
    #[inline(always)]
    pub const fn set_RESUME_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("OK_STATUS_3V", &self.OK_STATUS_3V())
            .field("HOSTDISCONDETECT_STATUS", &self.HOSTDISCONDETECT_STATUS())
            .field("DEVPLUGIN_STATUS", &self.DEVPLUGIN_STATUS())
            .field("RESUME_STATUS", &self.RESUME_STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ OK_STATUS_3V: {=bool:?}, HOSTDISCONDETECT_STATUS: {:?}, DEVPLUGIN_STATUS: {:?}, RESUME_STATUS: {=bool:?} }}",
            self.OK_STATUS_3V(),
            self.HOSTDISCONDETECT_STATUS(),
            self.DEVPLUGIN_STATUS(),
            self.RESUME_STATUS()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX(pub u32);
impl TX {
    #[doc = "Decode to trim the nominal 17."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> super::vals::TX_D_CAL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TX_D_CAL::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: super::vals::TX_D_CAL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_TXENCAL45DN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_TXENCAL45DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for TX {
    #[inline(always)]
    fn default() -> TX {
        TX(0)
    }
}
impl core::fmt::Debug for TX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DM", &self.TXCAL45DM())
            .field("TXENCAL45DN", &self.TXENCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .field("TXENCAL45DP", &self.TXENCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX {{ D_CAL: {:?}, TXCAL45DM: {=u8:?}, TXENCAL45DN: {=bool:?}, TXCAL45DP: {=u8:?}, TXENCAL45DP: {=bool:?} }}",
            self.D_CAL(),
            self.TXCAL45DM(),
            self.TXENCAL45DN(),
            self.TXCAL45DP(),
            self.TXENCAL45DP()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_CLR(pub u32);
impl TX_CLR {
    #[doc = "Decode to trim the nominal 17."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> super::vals::TX_CLR_D_CAL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TX_CLR_D_CAL::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: super::vals::TX_CLR_D_CAL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_TXENCAL45DN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_TXENCAL45DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for TX_CLR {
    #[inline(always)]
    fn default() -> TX_CLR {
        TX_CLR(0)
    }
}
impl core::fmt::Debug for TX_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CLR")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DM", &self.TXCAL45DM())
            .field("TXENCAL45DN", &self.TXENCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .field("TXENCAL45DP", &self.TXENCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_CLR {{ D_CAL: {:?}, TXCAL45DM: {=u8:?}, TXENCAL45DN: {=bool:?}, TXCAL45DP: {=u8:?}, TXENCAL45DP: {=bool:?} }}",
            self.D_CAL(),
            self.TXCAL45DM(),
            self.TXENCAL45DN(),
            self.TXCAL45DP(),
            self.TXENCAL45DP()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_SET(pub u32);
impl TX_SET {
    #[doc = "Decode to trim the nominal 17."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> super::vals::TX_SET_D_CAL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TX_SET_D_CAL::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: super::vals::TX_SET_D_CAL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_TXENCAL45DN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_TXENCAL45DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for TX_SET {
    #[inline(always)]
    fn default() -> TX_SET {
        TX_SET(0)
    }
}
impl core::fmt::Debug for TX_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_SET")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DM", &self.TXCAL45DM())
            .field("TXENCAL45DN", &self.TXENCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .field("TXENCAL45DP", &self.TXENCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_SET {{ D_CAL: {:?}, TXCAL45DM: {=u8:?}, TXENCAL45DN: {=bool:?}, TXCAL45DP: {=u8:?}, TXENCAL45DP: {=bool:?} }}",
            self.D_CAL(),
            self.TXCAL45DM(),
            self.TXENCAL45DN(),
            self.TXCAL45DP(),
            self.TXENCAL45DP()
        )
    }
}
#[doc = "USB PHY Transmitter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_TOG(pub u32);
impl TX_TOG {
    #[doc = "Decode to trim the nominal 17."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> super::vals::TX_TOG_D_CAL {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::TX_TOG_D_CAL::from_bits(val as u8)
    }
    #[doc = "Decode to trim the nominal 17."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: super::vals::TX_TOG_D_CAL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Enable resistance calibration on DN."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DN."]
    #[inline(always)]
    pub const fn set_TXENCAL45DN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Enable resistance calibration on DP."]
    #[must_use]
    #[inline(always)]
    pub const fn TXENCAL45DP(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resistance calibration on DP."]
    #[inline(always)]
    pub const fn set_TXENCAL45DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for TX_TOG {
    #[inline(always)]
    fn default() -> TX_TOG {
        TX_TOG(0)
    }
}
impl core::fmt::Debug for TX_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_TOG")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DM", &self.TXCAL45DM())
            .field("TXENCAL45DN", &self.TXENCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .field("TXENCAL45DP", &self.TXENCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_TOG {{ D_CAL: {:?}, TXCAL45DM: {=u8:?}, TXENCAL45DN: {=bool:?}, TXCAL45DP: {=u8:?}, TXENCAL45DP: {=bool:?} }}",
            self.D_CAL(),
            self.TXCAL45DM(),
            self.TXENCAL45DN(),
            self.TXCAL45DP(),
            self.TXENCAL45DP()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT(pub u32);
impl USB1_VBUS_DETECT {
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> super::vals::USB1_VBUS_DETECT_VBUSVALID_THRESH {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_VBUSVALID_THRESH::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_VBUSVALID_THRESH,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> super::vals::USB1_VBUS_DETECT_VBUSVALID_SEL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_VBUSVALID_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(&mut self, val: super::vals::USB1_VBUS_DETECT_VBUSVALID_SEL) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> super::vals::USB1_VBUS_DETECT_VBUS_SOURCE_SEL {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::USB1_VBUS_DETECT_VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_VBUS_SOURCE_SEL,
    ) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_EXT_ID_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_EXT_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_SESSVALID(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_SESSVALID(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_VBUSVALID_TO_SESSVALID,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRUP_CMPS(&self) -> super::vals::USB1_VBUS_DETECT_PWRUP_CMPS {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_PWRUP_CMPS::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[inline(always)]
    pub const fn set_PWRUP_CMPS(&mut self, val: super::vals::USB1_VBUS_DETECT_PWRUP_CMPS) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> super::vals::USB1_VBUS_DETECT_DISCHARGE_VBUS {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_DISCHARGE_VBUS::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(&mut self, val: super::vals::USB1_VBUS_DETECT_DISCHARGE_VBUS) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT {
        USB1_VBUS_DETECT(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_SESSVALID", &self.VBUSVALID_TO_SESSVALID())
            .field("PWRUP_CMPS", &self.PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT {{ VBUSVALID_THRESH: {:?}, VBUS_OVERRIDE_EN: {:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {:?}, VBUS_SOURCE_SEL: {:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {:?}, EXT_VBUS_OVERRIDE_EN: {:?}, VBUSVALID_TO_SESSVALID: {:?}, PWRUP_CMPS: {:?}, DISCHARGE_VBUS: {:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_SESSVALID(),
            self.PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_CLR(pub u32);
impl USB1_VBUS_DETECT_CLR {
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_THRESH,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_SEL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_SEL,
    ) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> super::vals::USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_VBUS_SOURCE_SEL,
    ) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_EXT_ID_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_EXT_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_SESSVALID(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_SESSVALID(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_VBUSVALID_TO_SESSVALID,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRUP_CMPS(&self) -> super::vals::USB1_VBUS_DETECT_CLR_PWRUP_CMPS {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_CLR_PWRUP_CMPS::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[inline(always)]
    pub const fn set_PWRUP_CMPS(&mut self, val: super::vals::USB1_VBUS_DETECT_CLR_PWRUP_CMPS) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> super::vals::USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_CLR_DISCHARGE_VBUS,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_CLR {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_CLR {
        USB1_VBUS_DETECT_CLR(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_CLR")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_SESSVALID", &self.VBUSVALID_TO_SESSVALID())
            .field("PWRUP_CMPS", &self.PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_CLR {{ VBUSVALID_THRESH: {:?}, VBUS_OVERRIDE_EN: {:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {:?}, VBUS_SOURCE_SEL: {:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {:?}, EXT_VBUS_OVERRIDE_EN: {:?}, VBUSVALID_TO_SESSVALID: {:?}, PWRUP_CMPS: {:?}, DISCHARGE_VBUS: {:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_SESSVALID(),
            self.PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_SET(pub u32);
impl USB1_VBUS_DETECT_SET {
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_THRESH {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_THRESH::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_THRESH,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_SEL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_SEL,
    ) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> super::vals::USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_VBUS_SOURCE_SEL,
    ) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pinmuxed value:."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_EXT_ID_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pinmuxed value."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_EXT_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_SESSVALID(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_SESSVALID(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_VBUSVALID_TO_SESSVALID,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRUP_CMPS(&self) -> super::vals::USB1_VBUS_DETECT_SET_PWRUP_CMPS {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_SET_PWRUP_CMPS::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator: Powers up the comparator used for the VBUS_VALID detector."]
    #[inline(always)]
    pub const fn set_PWRUP_CMPS(&mut self, val: super::vals::USB1_VBUS_DETECT_SET_PWRUP_CMPS) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> super::vals::USB1_VBUS_DETECT_SET_DISCHARGE_VBUS {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_SET_DISCHARGE_VBUS::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_SET_DISCHARGE_VBUS,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_SET {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_SET {
        USB1_VBUS_DETECT_SET(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_SET")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_SESSVALID", &self.VBUSVALID_TO_SESSVALID())
            .field("PWRUP_CMPS", &self.PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_SET {{ VBUSVALID_THRESH: {:?}, VBUS_OVERRIDE_EN: {:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {:?}, VBUS_SOURCE_SEL: {:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {:?}, EXT_VBUS_OVERRIDE_EN: {:?}, VBUSVALID_TO_SESSVALID: {:?}, PWRUP_CMPS: {:?}, DISCHARGE_VBUS: {:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_SESSVALID(),
            self.PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "USB PHY VBUS Detect Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_TOG(pub u32);
impl USB1_VBUS_DETECT_TOG {
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH::from_bits(val as u8)
    }
    #[doc = "Sets the threshold for the VBUSVALID comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_THRESH,
    ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS detect signal override enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "VBUS detect signal override enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for SESSEND The bit field provides the value for USB1_VBUS_DET_STAT\\[0\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for B-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[1\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for A-Device Session Valid The bit field provides the value for USB1_VBUS_DET_STAT\\[2\\] if USB_VBUS_DETECT\\[3\\] is set to value 1'b1."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for VBUS_VALID signal sent to USB controller The bit field provides the value for VBUS_VALID reported to the USB controller if the value of USB1_VBUS_DETECT\\[3\\] is set to 1'b1."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_SEL {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_SEL,
    ) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> super::vals::USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "Selects the source of the VBUS_VALID signal reported to the USB controller This is one of the bit fields that selects the source of the VBUS_VALID signal reported to the USB controller."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_VBUS_SOURCE_SEL,
    ) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable ID override using the register field. This bit is only used if EXT_ID_OVERRIDE_EN = 1'b0."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID override value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID override value."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable ID override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> super::vals::USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable ID override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_EXT_ID_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable VBUS override using the pin muxed value."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_EXT_VBUS_OVERRIDE_EN,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_SESSVALID(
        &self,
    ) -> super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID::from_bits(val as u8)
    }
    #[doc = "Selects the comparator used for VBUS_VALID This bit field controls the comparator used to report the VBUS_VALID results in USB1_VBUS_DETECT\\[3\\] between the VBUS_VALID comparator and the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_SESSVALID(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_VBUSVALID_TO_SESSVALID,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRUP_CMPS(&self) -> super::vals::USB1_VBUS_DETECT_TOG_PWRUP_CMPS {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::USB1_VBUS_DETECT_TOG_PWRUP_CMPS::from_bits(val as u8)
    }
    #[doc = "Enables the VBUS_VALID comparator Powers up the comparator used for the VBUS_VALID detector."]
    #[inline(always)]
    pub const fn set_PWRUP_CMPS(&mut self, val: super::vals::USB1_VBUS_DETECT_TOG_PWRUP_CMPS) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> super::vals::USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS::from_bits(val as u8)
    }
    #[doc = "Controls VBUS discharge resistor This bit field controls a nominal 22kohm resistor between the USB1_VBUS pin and ground."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(
        &mut self,
        val: super::vals::USB1_VBUS_DETECT_TOG_DISCHARGE_VBUS,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_TOG {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_TOG {
        USB1_VBUS_DETECT_TOG(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_TOG")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_SESSVALID", &self.VBUSVALID_TO_SESSVALID())
            .field("PWRUP_CMPS", &self.PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_TOG {{ VBUSVALID_THRESH: {:?}, VBUS_OVERRIDE_EN: {:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {:?}, VBUS_SOURCE_SEL: {:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {:?}, EXT_VBUS_OVERRIDE_EN: {:?}, VBUSVALID_TO_SESSVALID: {:?}, PWRUP_CMPS: {:?}, DISCHARGE_VBUS: {:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_SESSVALID(),
            self.PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "USB PHY VBUS Detector Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DET_STAT(pub u32);
impl USB1_VBUS_DET_STAT {
    #[doc = "Session End indicator Session End status, value inverted from Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND(&self) -> super::vals::SESSEND {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SESSEND::from_bits(val as u8)
    }
    #[doc = "Session End indicator Session End status, value inverted from Session Valid comparator."]
    #[inline(always)]
    pub const fn set_SESSEND(&mut self, val: super::vals::SESSEND) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID(&self) -> super::vals::BVALID {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BVALID::from_bits(val as u8)
    }
    #[doc = "B-Device Session Valid status B-Device Session Valid status, determined by the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_BVALID(&mut self, val: super::vals::BVALID) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID(&self) -> super::vals::AVALID {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AVALID::from_bits(val as u8)
    }
    #[doc = "A-Device Session Valid status A-Device Session Valid status, determined by the Session Valid comparator."]
    #[inline(always)]
    pub const fn set_AVALID(&mut self, val: super::vals::AVALID) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID(&self) -> super::vals::VBUS_VALID {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VBUS_VALID::from_bits(val as u8)
    }
    #[doc = "VBUS voltage status This bit field shows the result of VBUS_VALID detection for the USB1_VBUS pin."]
    #[inline(always)]
    pub const fn set_VBUS_VALID(&mut self, val: super::vals::VBUS_VALID) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID_3V(&self) -> super::vals::VBUS_VALID_3V {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VBUS_VALID_3V::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID_3V detector status The VBUS_VALID_3V detector has a lower threshold for the voltage on the USB1_VBUS pin than either the Session Valid or VBUS_VALID comparators."]
    #[inline(always)]
    pub const fn set_VBUS_VALID_3V(&mut self, val: super::vals::VBUS_VALID_3V) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1_VBUS_DET_STAT {
    #[inline(always)]
    fn default() -> USB1_VBUS_DET_STAT {
        USB1_VBUS_DET_STAT(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DET_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT")
            .field("SESSEND", &self.SESSEND())
            .field("BVALID", &self.BVALID())
            .field("AVALID", &self.AVALID())
            .field("VBUS_VALID", &self.VBUS_VALID())
            .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DET_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DET_STAT {{ SESSEND: {:?}, BVALID: {:?}, AVALID: {:?}, VBUS_VALID: {:?}, VBUS_VALID_3V: {:?} }}",
            self.SESSEND(),
            self.BVALID(),
            self.AVALID(),
            self.VBUS_VALID(),
            self.VBUS_VALID_3V()
        )
    }
}
