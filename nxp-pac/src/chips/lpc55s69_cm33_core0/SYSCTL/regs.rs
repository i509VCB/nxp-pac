#[doc = "Selects the source for SCK going into Flexcomm index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCCTRLSEL(pub u32);
impl FCCTRLSEL {
    #[doc = "Selects the source for SCK going into this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn SCKINSEL(&self) -> super::vals::SCKINSEL {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SCKINSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub const fn set_SCKINSEL(&mut self, val: super::vals::SCKINSEL) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects the source for WS going into this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn WSINSEL(&self) -> super::vals::WSINSEL {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::WSINSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub const fn set_WSINSEL(&mut self, val: super::vals::WSINSEL) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects the source for DATA input to this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAINSEL(&self) -> super::vals::DATAINSEL {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::DATAINSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub const fn set_DATAINSEL(&mut self, val: super::vals::DATAINSEL) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects the source for DATA output from this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn DATAOUTSEL(&self) -> super::vals::DATAOUTSEL {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::DATAOUTSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub const fn set_DATAOUTSEL(&mut self, val: super::vals::DATAOUTSEL) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for FCCTRLSEL {
    #[inline(always)]
    fn default() -> FCCTRLSEL {
        FCCTRLSEL(0)
    }
}
impl core::fmt::Debug for FCCTRLSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCCTRLSEL")
            .field("SCKINSEL", &self.SCKINSEL())
            .field("WSINSEL", &self.WSINSEL())
            .field("DATAINSEL", &self.DATAINSEL())
            .field("DATAOUTSEL", &self.DATAOUTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCCTRLSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCCTRLSEL {{ SCKINSEL: {:?}, WSINSEL: {:?}, DATAINSEL: {:?}, DATAOUTSEL: {:?} }}",
            self.SCKINSEL(),
            self.WSINSEL(),
            self.DATAINSEL(),
            self.DATAOUTSEL()
        )
    }
}
#[doc = "Selects sources and data combinations for shared signal set index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHAREDCTRLSET(pub u32);
impl SHAREDCTRLSET {
    #[doc = "Selects the source for SCK of this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn SHAREDSCKSEL(&self) -> super::vals::SHAREDSCKSEL {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SHAREDSCKSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub const fn set_SHAREDSCKSEL(&mut self, val: super::vals::SHAREDSCKSEL) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Selects the source for WS of this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn SHAREDWSSEL(&self) -> super::vals::SHAREDWSSEL {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::SHAREDWSSEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub const fn set_SHAREDWSSEL(&mut self, val: super::vals::SHAREDWSSEL) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Selects the source for DATA input for this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn SHAREDDATASEL(&self) -> super::vals::SHAREDDATASEL {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::SHAREDDATASEL::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub const fn set_SHAREDDATASEL(&mut self, val: super::vals::SHAREDDATASEL) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC0DATAOUTEN(&self) -> super::vals::FC0DATAOUTEN {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::FC0DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC0DATAOUTEN(&mut self, val: super::vals::FC0DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC1DATAOUTEN(&self) -> super::vals::FC1DATAOUTEN {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::FC1DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC1DATAOUTEN(&mut self, val: super::vals::FC1DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC2DATAOUTEN(&self) -> super::vals::FC2DATAOUTEN {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::FC2DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC2DATAOUTEN(&mut self, val: super::vals::FC2DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC4DATAOUTEN(&self) -> super::vals::FC4DATAOUTEN {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::FC4DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC4DATAOUTEN(&mut self, val: super::vals::FC4DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC5DATAOUTEN(&self) -> super::vals::FC5DATAOUTEN {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::FC5DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC5DATAOUTEN(&mut self, val: super::vals::FC5DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC6DATAOUTEN(&self) -> super::vals::FC6DATAOUTEN {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::FC6DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC6DATAOUTEN(&mut self, val: super::vals::FC6DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn FC7DATAOUTEN(&self) -> super::vals::FC7DATAOUTEN {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::FC7DATAOUTEN::from_bits(val as u8)
    }
    #[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_FC7DATAOUTEN(&mut self, val: super::vals::FC7DATAOUTEN) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for SHAREDCTRLSET {
    #[inline(always)]
    fn default() -> SHAREDCTRLSET {
        SHAREDCTRLSET(0)
    }
}
impl core::fmt::Debug for SHAREDCTRLSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHAREDCTRLSET")
            .field("SHAREDSCKSEL", &self.SHAREDSCKSEL())
            .field("SHAREDWSSEL", &self.SHAREDWSSEL())
            .field("SHAREDDATASEL", &self.SHAREDDATASEL())
            .field("FC0DATAOUTEN", &self.FC0DATAOUTEN())
            .field("FC1DATAOUTEN", &self.FC1DATAOUTEN())
            .field("FC2DATAOUTEN", &self.FC2DATAOUTEN())
            .field("FC4DATAOUTEN", &self.FC4DATAOUTEN())
            .field("FC5DATAOUTEN", &self.FC5DATAOUTEN())
            .field("FC6DATAOUTEN", &self.FC6DATAOUTEN())
            .field("FC7DATAOUTEN", &self.FC7DATAOUTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHAREDCTRLSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHAREDCTRLSET {{ SHAREDSCKSEL: {:?}, SHAREDWSSEL: {:?}, SHAREDDATASEL: {:?}, FC0DATAOUTEN: {:?}, FC1DATAOUTEN: {:?}, FC2DATAOUTEN: {:?}, FC4DATAOUTEN: {:?}, FC5DATAOUTEN: {:?}, FC6DATAOUTEN: {:?}, FC7DATAOUTEN: {:?} }}",
            self.SHAREDSCKSEL(),
            self.SHAREDWSSEL(),
            self.SHAREDDATASEL(),
            self.FC0DATAOUTEN(),
            self.FC1DATAOUTEN(),
            self.FC2DATAOUTEN(),
            self.FC4DATAOUTEN(),
            self.FC5DATAOUTEN(),
            self.FC6DATAOUTEN(),
            self.FC7DATAOUTEN()
        )
    }
}
#[doc = "update lock out control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UPDATELCKOUT(pub u32);
impl UPDATELCKOUT {
    #[doc = "All Registers."]
    #[must_use]
    #[inline(always)]
    pub const fn UPDATELCKOUT(&self) -> super::vals::UPDATELCKOUT {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UPDATELCKOUT::from_bits(val as u8)
    }
    #[doc = "All Registers."]
    #[inline(always)]
    pub const fn set_UPDATELCKOUT(&mut self, val: super::vals::UPDATELCKOUT) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UPDATELCKOUT {
    #[inline(always)]
    fn default() -> UPDATELCKOUT {
        UPDATELCKOUT(0)
    }
}
impl core::fmt::Debug for UPDATELCKOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATELCKOUT")
            .field("UPDATELCKOUT", &self.UPDATELCKOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UPDATELCKOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UPDATELCKOUT {{ UPDATELCKOUT: {:?} }}",
            self.UPDATELCKOUT()
        )
    }
}
#[doc = "Status register for USB HS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB_HS_STATUS(pub u32);
impl USB_HS_STATUS {
    #[doc = "USB_HS: Low voltage detection on 3.3V supply."]
    #[must_use]
    #[inline(always)]
    pub const fn USBHS_3V_NOK(&self) -> super::vals::USBHS_3V_NOK {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::USBHS_3V_NOK::from_bits(val as u8)
    }
    #[doc = "USB_HS: Low voltage detection on 3.3V supply."]
    #[inline(always)]
    pub const fn set_USBHS_3V_NOK(&mut self, val: super::vals::USBHS_3V_NOK) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for USB_HS_STATUS {
    #[inline(always)]
    fn default() -> USB_HS_STATUS {
        USB_HS_STATUS(0)
    }
}
impl core::fmt::Debug for USB_HS_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_HS_STATUS")
            .field("USBHS_3V_NOK", &self.USBHS_3V_NOK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB_HS_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB_HS_STATUS {{ USBHS_3V_NOK: {:?} }}",
            self.USBHS_3V_NOK()
        )
    }
}
