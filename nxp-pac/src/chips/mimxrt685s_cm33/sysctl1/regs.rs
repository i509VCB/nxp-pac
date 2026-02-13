#[doc = "DSP NMI source selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspnmisrcsel(pub u32);
impl Dspnmisrcsel {
    #[doc = "Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
    #[must_use]
    #[inline(always)]
    pub const fn nmisrcsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects one of the DSP interrupt sources as the NMI source. See DSP Interrupt Slot Table for Interrupt Slot Numers."]
    #[inline(always)]
    pub const fn set_nmisrcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "NMI interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nmien(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "NMI interrupt enable"]
    #[inline(always)]
    pub const fn set_nmien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dspnmisrcsel {
    #[inline(always)]
    fn default() -> Dspnmisrcsel {
        Dspnmisrcsel(0)
    }
}
impl core::fmt::Debug for Dspnmisrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspnmisrcsel")
            .field("nmisrcsel", &self.nmisrcsel())
            .field("nmien", &self.nmien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspnmisrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dspnmisrcsel {{ nmisrcsel: {=u8:?}, nmien: {=bool:?} }}",
            self.nmisrcsel(),
            self.nmien()
        )
    }
}
#[doc = "flexcomm control selection N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcctrlsel(pub u32);
impl Fcctrlsel {
    #[doc = "SCK IN Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sckinsel(&self) -> super::vals::Sckinsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sckinsel::from_bits(val as u8)
    }
    #[doc = "SCK IN Select. . ."]
    #[inline(always)]
    pub const fn set_sckinsel(&mut self, val: super::vals::Sckinsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "WS IN Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn wsinsel(&self) -> super::vals::Wsinsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Wsinsel::from_bits(val as u8)
    }
    #[doc = "WS IN Select. . ."]
    #[inline(always)]
    pub const fn set_wsinsel(&mut self, val: super::vals::Wsinsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DATA IN Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn datainsel(&self) -> super::vals::Datainsel {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Datainsel::from_bits(val as u8)
    }
    #[doc = "DATA IN Select. . ."]
    #[inline(always)]
    pub const fn set_datainsel(&mut self, val: super::vals::Datainsel) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DATA OUT Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn dataoutsel(&self) -> super::vals::Dataoutsel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Dataoutsel::from_bits(val as u8)
    }
    #[doc = "DATA OUT Select. . ."]
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
#[doc = "mclk direction control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mclkpindir(pub u32);
impl Mclkpindir {
    #[doc = "Selects one of the M33 interrupt sources"]
    #[must_use]
    #[inline(always)]
    pub const fn mclkpindir(&self) -> super::vals::Mclkpindir {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mclkpindir::from_bits(val as u8)
    }
    #[doc = "Selects one of the M33 interrupt sources"]
    #[inline(always)]
    pub const fn set_mclkpindir(&mut self, val: super::vals::Mclkpindir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Mclkpindir {
    #[inline(always)]
    fn default() -> Mclkpindir {
        Mclkpindir(0)
    }
}
impl core::fmt::Debug for Mclkpindir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mclkpindir")
            .field("mclkpindir", &self.mclkpindir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mclkpindir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mclkpindir {{ mclkpindir: {:?} }}", self.mclkpindir())
    }
}
#[doc = "RX Event Pulse Generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxevpulsegen(pub u32);
impl Rxevpulsegen {
    #[doc = "RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn rxevpulsegen(&self) -> super::vals::Rxevpulsegen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rxevpulsegen::from_bits(val as u8)
    }
    #[doc = "RX Event Pulse Generator. Writing a '1' to this register will create a one PSCLK pulse width of logic '1'. It is automatically cleared."]
    #[inline(always)]
    pub const fn set_rxevpulsegen(&mut self, val: super::vals::Rxevpulsegen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxevpulsegen {
    #[inline(always)]
    fn default() -> Rxevpulsegen {
        Rxevpulsegen(0)
    }
}
impl core::fmt::Debug for Rxevpulsegen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxevpulsegen")
            .field("rxevpulsegen", &self.rxevpulsegen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxevpulsegen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rxevpulsegen {{ rxevpulsegen: {:?} }}",
            self.rxevpulsegen()
        )
    }
}
#[doc = "shared control set N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sharedctrlset(pub u32);
impl Sharedctrlset {
    #[doc = "Shared SCK Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sharedscksel(&self) -> super::vals::Sharedscksel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sharedscksel::from_bits(val as u8)
    }
    #[doc = "Shared SCK Select. . ."]
    #[inline(always)]
    pub const fn set_sharedscksel(&mut self, val: super::vals::Sharedscksel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Shared WS Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sharedwssel(&self) -> super::vals::Sharedwssel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sharedwssel::from_bits(val as u8)
    }
    #[doc = "Shared WS Select. . ."]
    #[inline(always)]
    pub const fn set_sharedwssel(&mut self, val: super::vals::Sharedwssel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Shared DATA Select. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn shareddatasel(&self) -> super::vals::Shareddatasel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Shareddatasel::from_bits(val as u8)
    }
    #[doc = "Shared DATA Select. . ."]
    #[inline(always)]
    pub const fn set_shareddatasel(&mut self, val: super::vals::Shareddatasel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0dataouten(&self) -> super::vals::Fc0dataouten {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fc0dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc0dataouten(&mut self, val: super::vals::Fc0dataouten) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1dataouten(&self) -> super::vals::Fc1dataouten {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Fc1dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc1dataouten(&mut self, val: super::vals::Fc1dataouten) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn f20dataouten(&self) -> super::vals::F20dataouten {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::F20dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_f20dataouten(&mut self, val: super::vals::F20dataouten) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3dataouten(&self) -> super::vals::Fc3dataouten {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Fc3dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc3dataouten(&mut self, val: super::vals::Fc3dataouten) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4dataouten(&self) -> super::vals::Fc4dataouten {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Fc4dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc4dataouten(&mut self, val: super::vals::Fc4dataouten) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5dataouten(&self) -> super::vals::Fc5dataouten {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Fc5dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc5dataouten(&mut self, val: super::vals::Fc5dataouten) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6dataouten(&self) -> super::vals::Fc6dataouten {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fc6dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 DATAOUT OUTPUT ENABLE"]
    #[inline(always)]
    pub const fn set_fc6dataouten(&mut self, val: super::vals::Fc6dataouten) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7dataouten(&self) -> super::vals::Fc7dataouten {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Fc7dataouten::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 DATAOUT OUTPUT ENABLE"]
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
            .field("f20dataouten", &self.f20dataouten())
            .field("fc3dataouten", &self.fc3dataouten())
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
            "Sharedctrlset {{ sharedscksel: {:?}, sharedwssel: {:?}, shareddatasel: {:?}, fc0dataouten: {:?}, fc1dataouten: {:?}, f20dataouten: {:?}, fc3dataouten: {:?}, fc4dataouten: {:?}, fc5dataouten: {:?}, fc6dataouten: {:?}, fc7dataouten: {:?} }}",
            self.sharedscksel(),
            self.sharedwssel(),
            self.shareddatasel(),
            self.fc0dataouten(),
            self.fc1dataouten(),
            self.f20dataouten(),
            self.fc3dataouten(),
            self.fc4dataouten(),
            self.fc5dataouten(),
            self.fc6dataouten(),
            self.fc7dataouten()
        )
    }
}
