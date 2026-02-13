#[doc = "Selects the source for SCK going into Flexcomm index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcctrlsel(pub u32);
impl Fcctrlsel {
    #[doc = "Selects the source for SCK going into this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn sckinsel(&self) -> super::vals::Sckinsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sckinsel::from_bits(val as u8)
    }
    #[doc = "Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub const fn set_sckinsel(&mut self, val: super::vals::Sckinsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects the source for WS going into this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn wsinsel(&self) -> super::vals::Wsinsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Wsinsel::from_bits(val as u8)
    }
    #[doc = "Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub const fn set_wsinsel(&mut self, val: super::vals::Wsinsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects the source for DATA input to this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn datainsel(&self) -> super::vals::Datainsel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Datainsel::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub const fn set_datainsel(&mut self, val: super::vals::Datainsel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects the source for DATA output from this Flexcomm."]
    #[must_use]
    #[inline(always)]
    pub const fn dataoutsel(&self) -> super::vals::Dataoutsel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Dataoutsel::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub const fn set_dataoutsel(&mut self, val: super::vals::Dataoutsel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Fcctrlsel {
    #[inline(always)]
    fn default() -> Fcctrlsel {
        Fcctrlsel(0)
    }
}
impl core::fmt::Debug for Fcctrlsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcctrlsel")
            .field("sckinsel", &self.sckinsel())
            .field("wsinsel", &self.wsinsel())
            .field("datainsel", &self.datainsel())
            .field("dataoutsel", &self.dataoutsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcctrlsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcctrlsel {{ sckinsel: {:?}, wsinsel: {:?}, datainsel: {:?}, dataoutsel: {:?} }}",
            self.sckinsel(),
            self.wsinsel(),
            self.datainsel(),
            self.dataoutsel()
        )
    }
}
#[doc = "Selects sources and data combinations for shared signal set index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sharedctrlset(pub u32);
impl Sharedctrlset {
    #[doc = "Selects the source for SCK of this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn sharedscksel(&self) -> super::vals::Sharedscksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sharedscksel::from_bits(val as u8)
    }
    #[doc = "Selects the source for SCK of this shared signal set."]
    #[inline(always)]
    pub const fn set_sharedscksel(&mut self, val: super::vals::Sharedscksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Selects the source for WS of this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn sharedwssel(&self) -> super::vals::Sharedwssel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sharedwssel::from_bits(val as u8)
    }
    #[doc = "Selects the source for WS of this shared signal set."]
    #[inline(always)]
    pub const fn set_sharedwssel(&mut self, val: super::vals::Sharedwssel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Selects the source for DATA input for this shared signal set."]
    #[must_use]
    #[inline(always)]
    pub const fn shareddatasel(&self) -> super::vals::Shareddatasel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Shareddatasel::from_bits(val as u8)
    }
    #[doc = "Selects the source for DATA input for this shared signal set."]
    #[inline(always)]
    pub const fn set_shareddatasel(&mut self, val: super::vals::Shareddatasel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc0dataouten(&self) -> super::vals::Fc0dataouten {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fc0dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC0 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc0dataouten(&mut self, val: super::vals::Fc0dataouten) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc1dataouten(&self) -> super::vals::Fc1dataouten {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Fc1dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC1 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc1dataouten(&mut self, val: super::vals::Fc1dataouten) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc2dataouten(&self) -> super::vals::Fc2dataouten {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Fc2dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC2 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc2dataouten(&mut self, val: super::vals::Fc2dataouten) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc4dataouten(&self) -> super::vals::Fc4dataouten {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Fc4dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC4 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc4dataouten(&mut self, val: super::vals::Fc4dataouten) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc5dataouten(&self) -> super::vals::Fc5dataouten {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Fc5dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC5 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc5dataouten(&mut self, val: super::vals::Fc5dataouten) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc6dataouten(&self) -> super::vals::Fc6dataouten {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fc6dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC6 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc6dataouten(&mut self, val: super::vals::Fc6dataouten) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[must_use]
    #[inline(always)]
    pub const fn fc7dataouten(&self) -> super::vals::Fc7dataouten {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Fc7dataouten::from_bits(val as u8)
    }
    #[doc = "Controls FC7 contribution to SHAREDDATAOUT for this shared set."]
    #[inline(always)]
    pub const fn set_fc7dataouten(&mut self, val: super::vals::Fc7dataouten) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Sharedctrlset {
    #[inline(always)]
    fn default() -> Sharedctrlset {
        Sharedctrlset(0)
    }
}
impl core::fmt::Debug for Sharedctrlset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sharedctrlset")
            .field("sharedscksel", &self.sharedscksel())
            .field("sharedwssel", &self.sharedwssel())
            .field("shareddatasel", &self.shareddatasel())
            .field("fc0dataouten", &self.fc0dataouten())
            .field("fc1dataouten", &self.fc1dataouten())
            .field("fc2dataouten", &self.fc2dataouten())
            .field("fc4dataouten", &self.fc4dataouten())
            .field("fc5dataouten", &self.fc5dataouten())
            .field("fc6dataouten", &self.fc6dataouten())
            .field("fc7dataouten", &self.fc7dataouten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sharedctrlset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sharedctrlset {{ sharedscksel: {:?}, sharedwssel: {:?}, shareddatasel: {:?}, fc0dataouten: {:?}, fc1dataouten: {:?}, fc2dataouten: {:?}, fc4dataouten: {:?}, fc5dataouten: {:?}, fc6dataouten: {:?}, fc7dataouten: {:?} }}",
            self.sharedscksel(),
            self.sharedwssel(),
            self.shareddatasel(),
            self.fc0dataouten(),
            self.fc1dataouten(),
            self.fc2dataouten(),
            self.fc4dataouten(),
            self.fc5dataouten(),
            self.fc6dataouten(),
            self.fc7dataouten()
        )
    }
}
#[doc = "update lock out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Updatelckout(pub u32);
impl Updatelckout {
    #[doc = "All Registers"]
    #[must_use]
    #[inline(always)]
    pub const fn updatelckout(&self) -> super::vals::Updatelckout {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Updatelckout::from_bits(val as u8)
    }
    #[doc = "All Registers"]
    #[inline(always)]
    pub const fn set_updatelckout(&mut self, val: super::vals::Updatelckout) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Updatelckout {
    #[inline(always)]
    fn default() -> Updatelckout {
        Updatelckout(0)
    }
}
impl core::fmt::Debug for Updatelckout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Updatelckout")
            .field("updatelckout", &self.updatelckout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Updatelckout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Updatelckout {{ updatelckout: {:?} }}",
            self.updatelckout()
        )
    }
}
#[doc = "Status register for USB HS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbHsStatus(pub u32);
impl UsbHsStatus {
    #[doc = "USB_HS: Low voltage detection on 3.3V supply."]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_3v_nok(&self) -> super::vals::Usbhs3vNok {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbhs3vNok::from_bits(val as u8)
    }
    #[doc = "USB_HS: Low voltage detection on 3.3V supply."]
    #[inline(always)]
    pub const fn set_usbhs_3v_nok(&mut self, val: super::vals::Usbhs3vNok) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbHsStatus {
    #[inline(always)]
    fn default() -> UsbHsStatus {
        UsbHsStatus(0)
    }
}
impl core::fmt::Debug for UsbHsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbHsStatus")
            .field("usbhs_3v_nok", &self.usbhs_3v_nok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbHsStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbHsStatus {{ usbhs_3v_nok: {:?} }}",
            self.usbhs_3v_nok()
        )
    }
}
