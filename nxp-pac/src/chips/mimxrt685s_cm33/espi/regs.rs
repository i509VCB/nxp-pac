#[doc = "Selects DMA for Ports (if used)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmactrl(pub u32);
impl Dmactrl {
    #[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0en(&self) -> super::vals::Dma0en {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dma0en::from_bits(val as u8)
    }
    #[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub const fn set_dma0en(&mut self, val: super::vals::Dma0en) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1en(&self) -> super::vals::Dma1en {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Dma1en::from_bits(val as u8)
    }
    #[doc = "Enables the DMA use on the 1st channel for eSPI and selects what triggers it"]
    #[inline(always)]
    pub const fn set_dma1en(&mut self, val: super::vals::Dma1en) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0port(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub const fn set_dma0port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1port(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects which port is operating the DMA: Value Meaning 0 to 7 Is normal port number up to max ports (e"]
    #[inline(always)]
    pub const fn set_dma1port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub const fn set_cnt0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Used with Mailbox and Bus master to allow RAM contents to be copied"]
    #[inline(always)]
    pub const fn set_cnt1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Dmactrl {
    #[inline(always)]
    fn default() -> Dmactrl {
        Dmactrl(0)
    }
}
impl core::fmt::Debug for Dmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmactrl")
            .field("dma0en", &self.dma0en())
            .field("dma1en", &self.dma1en())
            .field("dma0port", &self.dma0port())
            .field("dma1port", &self.dma1port())
            .field("cnt0", &self.cnt0())
            .field("cnt1", &self.cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmactrl {{ dma0en: {:?}, dma1en: {:?}, dma0port: {=u8:?}, dma1port: {=u8:?}, cnt0: {=u8:?}, cnt1: {=u8:?} }}",
            self.dma0en(),
            self.dma1en(),
            self.dma0port(),
            self.dma1port(),
            self.cnt0(),
            self.cnt1()
        )
    }
}
#[doc = "eSPI Capabilities of MCU to send to Host"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espicap(pub u32);
impl Espicap {
    #[doc = "SPI mode allowed (host still has to select):"]
    #[must_use]
    #[inline(always)]
    pub const fn spicap(&self) -> super::vals::Spicap {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Spicap::from_bits(val as u8)
    }
    #[doc = "SPI mode allowed (host still has to select):"]
    #[inline(always)]
    pub const fn set_spicap(&mut self, val: super::vals::Spicap) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Maximum SPI Clock speed to allow (host still chooses):"]
    #[must_use]
    #[inline(always)]
    pub const fn maxspd(&self) -> super::vals::Maxspd {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Maxspd::from_bits(val as u8)
    }
    #[doc = "Maximum SPI Clock speed to allow (host still chooses):"]
    #[inline(always)]
    pub const fn set_maxspd(&mut self, val: super::vals::Maxspd) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Allow Alert to be a pin if the Host wants"]
    #[must_use]
    #[inline(always)]
    pub const fn alpin(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Alert to be a pin if the Host wants"]
    #[inline(always)]
    pub const fn set_alpin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "If 1, allow OOB"]
    #[must_use]
    #[inline(always)]
    pub const fn oobok(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, allow OOB"]
    #[inline(always)]
    pub const fn set_oobok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
    #[must_use]
    #[inline(always)]
    pub const fn memmx(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, allow 128 byte payload for memory and OOB access, else limit to 64"]
    #[inline(always)]
    pub const fn set_memmx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Selects max Flash payload size to allow:"]
    #[must_use]
    #[inline(always)]
    pub const fn flashmx(&self) -> super::vals::Flashmx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Flashmx::from_bits(val as u8)
    }
    #[doc = "Selects max Flash payload size to allow:"]
    #[inline(always)]
    pub const fn set_flashmx(&mut self, val: super::vals::Flashmx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "If 1, then Slave Attached Flash is possible with this firmware"]
    #[must_use]
    #[inline(always)]
    pub const fn saf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, then Slave Attached Flash is possible with this firmware"]
    #[inline(always)]
    pub const fn set_saf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Min Erase sector size allowed if SAF used"]
    #[must_use]
    #[inline(always)]
    pub const fn safera(&self) -> super::vals::Safera {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Safera::from_bits(val as u8)
    }
    #[doc = "Min Erase sector size allowed if SAF used"]
    #[inline(always)]
    pub const fn set_safera(&mut self, val: super::vals::Safera) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
}
impl Default for Espicap {
    #[inline(always)]
    fn default() -> Espicap {
        Espicap(0)
    }
}
impl core::fmt::Debug for Espicap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espicap")
            .field("spicap", &self.spicap())
            .field("maxspd", &self.maxspd())
            .field("alpin", &self.alpin())
            .field("oobok", &self.oobok())
            .field("memmx", &self.memmx())
            .field("flashmx", &self.flashmx())
            .field("saf", &self.saf())
            .field("safera", &self.safera())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espicap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espicap {{ spicap: {:?}, maxspd: {:?}, alpin: {=bool:?}, oobok: {=bool:?}, memmx: {=bool:?}, flashmx: {:?}, saf: {=bool:?}, safera: {:?} }}",
            self.spicap(),
            self.maxspd(),
            self.alpin(),
            self.oobok(),
            self.memmx(),
            self.flashmx(),
            self.saf(),
            self.safera()
        )
    }
}
#[doc = "eSPI Configuration settings from eSPI"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espicfg(pub u32);
impl Espicfg {
    #[doc = "If 1, will use 128 byte payload for memory and OOB access, else limited to 64"]
    #[must_use]
    #[inline(always)]
    pub const fn memsz(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, will use 128 byte payload for memory and OOB access, else limited to 64"]
    #[inline(always)]
    pub const fn set_memsz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates max Flash payload size selected using same values as FLASHMX"]
    #[must_use]
    #[inline(always)]
    pub const fn flashsz(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates max Flash payload size selected using same values as FLASHMX"]
    #[inline(always)]
    pub const fn set_flashsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "SPI Mode selected by Host"]
    #[must_use]
    #[inline(always)]
    pub const fn spimod(&self) -> super::vals::Spimod {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Spimod::from_bits(val as u8)
    }
    #[doc = "SPI Mode selected by Host"]
    #[inline(always)]
    pub const fn set_spimod(&mut self, val: super::vals::Spimod) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Alert is a pin vs. MISO"]
    #[must_use]
    #[inline(always)]
    pub const fn alert(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Alert is a pin vs. MISO"]
    #[inline(always)]
    pub const fn set_alert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Alert is OD as a pin"]
    #[must_use]
    #[inline(always)]
    pub const fn alertod(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Alert is OD as a pin"]
    #[inline(always)]
    pub const fn set_alertod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SPI Speed selected by Host. See note in section 2.2 on higher eSPI clock speeds and system setup."]
    #[must_use]
    #[inline(always)]
    pub const fn spispd(&self) -> super::vals::Spispd {
        let val = (self.0 >> 7usize) & 0x07;
        super::vals::Spispd::from_bits(val as u8)
    }
    #[doc = "SPI Speed selected by Host. See note in section 2.2 on higher eSPI clock speeds and system setup."]
    #[inline(always)]
    pub const fn set_spispd(&mut self, val: super::vals::Spispd) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "CRC checking is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CRC checking is enabled"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus Master is OK"]
    #[must_use]
    #[inline(always)]
    pub const fn busmok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Master is OK"]
    #[inline(always)]
    pub const fn set_busmok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 0 (memory) is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn memena(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 (memory) is enabled"]
    #[inline(always)]
    pub const fn set_memena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 1 (Vwire) is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn vwok(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 (Vwire) is enabled"]
    #[inline(always)]
    pub const fn set_vwok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 2 (OOB) is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn oobok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 (OOB) is enabled"]
    #[inline(always)]
    pub const fn set_oobok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Flash erase size (and if enabled):"]
    #[must_use]
    #[inline(always)]
    pub const fn flshera(&self) -> super::vals::Flshera {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::Flshera::from_bits(val as u8)
    }
    #[doc = "Flash erase size (and if enabled):"]
    #[inline(always)]
    pub const fn set_flshera(&mut self, val: super::vals::Flshera) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Channel 3 (Flash) is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn flshok(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 (Flash) is enabled."]
    #[inline(always)]
    pub const fn set_flshok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Is 1 if Slave Attached Flash is enabled. Only possible if SAF is set in ESPICAP."]
    #[must_use]
    #[inline(always)]
    pub const fn saf(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Is 1 if Slave Attached Flash is enabled. Only possible if SAF is set in ESPICAP."]
    #[inline(always)]
    pub const fn set_saf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Espicfg {
    #[inline(always)]
    fn default() -> Espicfg {
        Espicfg(0)
    }
}
impl core::fmt::Debug for Espicfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espicfg")
            .field("memsz", &self.memsz())
            .field("flashsz", &self.flashsz())
            .field("spimod", &self.spimod())
            .field("alert", &self.alert())
            .field("alertod", &self.alertod())
            .field("spispd", &self.spispd())
            .field("crc", &self.crc())
            .field("busmok", &self.busmok())
            .field("memena", &self.memena())
            .field("vwok", &self.vwok())
            .field("oobok", &self.oobok())
            .field("flshera", &self.flshera())
            .field("flshok", &self.flshok())
            .field("saf", &self.saf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espicfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espicfg {{ memsz: {=bool:?}, flashsz: {=u8:?}, spimod: {:?}, alert: {=bool:?}, alertod: {=bool:?}, spispd: {:?}, crc: {=bool:?}, busmok: {=bool:?}, memena: {=bool:?}, vwok: {=bool:?}, oobok: {=bool:?}, flshera: {:?}, flshok: {=bool:?}, saf: {=bool:?} }}",
            self.memsz(),
            self.flashsz(),
            self.spimod(),
            self.alert(),
            self.alertod(),
            self.spispd(),
            self.crc(),
            self.busmok(),
            self.memena(),
            self.vwok(),
            self.oobok(),
            self.flshera(),
            self.flshok(),
            self.saf()
        )
    }
}
#[doc = "Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espimisc(pub u32);
impl Espimisc {
    #[doc = "Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_oe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to make the Alert/Reset pin an output GPIO, else is input (High-Z)"]
    #[inline(always)]
    pub const fn set_gpio_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_od(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to make the Alert/Reset pin act open-drain when GPIO_OE=1"]
    #[inline(always)]
    pub const fn set_gpio_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Used when GPIO_OE=1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Used when GPIO_OE=1"]
    #[inline(always)]
    pub const fn set_gpio_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Is the current state of the Alert/Reset pin, whether in input mode or not"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_in(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Is the current state of the Alert/Reset pin, whether in input mode or not"]
    #[inline(always)]
    pub const fn set_gpio_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset# is a GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn risgp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset# is a GPIO"]
    #[inline(always)]
    pub const fn set_risgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit, when set, will employ clock gating for the eSPI side"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsav(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This bit, when set, will employ clock gating for the eSPI side"]
    #[inline(always)]
    pub const fn set_pwrsav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Espimisc {
    #[inline(always)]
    fn default() -> Espimisc {
        Espimisc(0)
    }
}
impl core::fmt::Debug for Espimisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espimisc")
            .field("gpio_oe", &self.gpio_oe())
            .field("gpio_od", &self.gpio_od())
            .field("gpio_out", &self.gpio_out())
            .field("gpio_in", &self.gpio_in())
            .field("risgp", &self.risgp())
            .field("pwrsav", &self.pwrsav())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espimisc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espimisc {{ gpio_oe: {=bool:?}, gpio_od: {=bool:?}, gpio_out: {=bool:?}, gpio_in: {=bool:?}, risgp: {=bool:?}, pwrsav: {=bool:?} }}",
            self.gpio_oe(),
            self.gpio_od(),
            self.gpio_out(),
            self.gpio_in(),
            self.risgp(),
            self.pwrsav()
        )
    }
}
#[doc = "Interrupt Clear (disable)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "If set to 1, clears corresponding port interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn port_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "If set to 1, clears corresponding port interrupt enable"]
    #[inline(always)]
    pub const fn set_port_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "If set to 1, clears Port80 interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears Port80 interrupt enable."]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If set to 1, clears Reset change interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_rst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears Reset change interrupt enable."]
    #[inline(always)]
    pub const fn set_bus_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If set to 1, clears IRQ completion interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_upd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears IRQ completion interrupt enable."]
    #[inline(always)]
    pub const fn set_irq_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "If set to 1, clears Wire Change interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wire_chg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears Wire Change interrupt enable."]
    #[inline(always)]
    pub const fn set_wire_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "If set to 1, clears HStall interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears HStall interrupt enable."]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "If set to 1, clears CRCERR interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears CRCERR interrupt enable."]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "If set to 1, clears GPIO interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, clears GPIO interrupt enable."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
impl core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr {{ port_int: {=u8:?}, p80int: {=bool:?}, bus_rst: {=bool:?}, irq_upd: {=bool:?}, wire_chg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?} }}",
            self.port_int(),
            self.p80int(),
            self.bus_rst(),
            self.irq_upd(),
            self.wire_chg(),
            self.hstall(),
            self.crcerr(),
            self.gpio()
        )
    }
}
#[doc = "Interrupt Set (enable)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[must_use]
    #[inline(always)]
    pub const fn port_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub const fn set_port_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "If set to 1, Port80 will interrupt main processor on update from Host."]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, Port80 will interrupt main processor on update from Host."]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If 1, a change in Bus Reset status will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_rst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, a change in Bus Reset status will interrupt main processor."]
    #[inline(always)]
    pub const fn set_bus_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, completion of an IRQ update will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_upd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, completion of an IRQ update will interrupt main processor."]
    #[inline(always)]
    pub const fn set_irq_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn wire_chg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_wire_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "If 1, when a CRC error detected, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when a CRC error detected, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
impl core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset {{ port_int: {=u8:?}, p80int: {=bool:?}, bus_rst: {=bool:?}, irq_upd: {=bool:?}, wire_chg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?} }}",
            self.port_int(),
            self.p80int(),
            self.bus_rst(),
            self.irq_upd(),
            self.wire_chg(),
            self.hstall(),
            self.crcerr(),
            self.gpio()
        )
    }
}
#[doc = "Masked interrupt status (causes)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[must_use]
    #[inline(always)]
    pub const fn port_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "If set to 1, corresponding port will interrupt main processor if matches IRule"]
    #[inline(always)]
    pub const fn set_port_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "If set to 1, Port80 will interrupt main processor on update from Host."]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, Port80 will interrupt main processor on update from Host."]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If 1, a change in Bus Reset status will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_rst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, a change in Bus Reset status will interrupt main processor."]
    #[inline(always)]
    pub const fn set_bus_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, completion of an IRQ update will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn irq_upd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, completion of an IRQ update will interrupt main processor."]
    #[inline(always)]
    pub const fn set_irq_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn wire_chg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when one or more VWire input has changed, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_wire_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when the HStall bit is set in the MSTAT register, will interrupt the main processor."]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "If 1, when a CRC error detected, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when a CRC error detected, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, when ESPICFG GPIO changes input value, will interrupt main processor."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ port_int: {=u8:?}, p80int: {=bool:?}, bus_rst: {=bool:?}, irq_upd: {=bool:?}, wire_chg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?} }}",
            self.port_int(),
            self.p80int(),
            self.bus_rst(),
            self.irq_upd(),
            self.wire_chg(),
            self.hstall(),
            self.crcerr(),
            self.gpio()
        )
    }
}
#[doc = "IRQ to drive into Host (with eSPI)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqpush(pub u32);
impl Irqpush {
    #[doc = "Set to the IRQ to push across to the Host."]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Set to the IRQ to push across to the Host."]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Will go to 1 when complete; this is just the same bit as IrqUpd in MSTAT, which can interrupt using INTENSET"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Will go to 1 when complete; this is just the same bit as IrqUpd in MSTAT, which can interrupt using INTENSET"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Irqpush {
    #[inline(always)]
    fn default() -> Irqpush {
        Irqpush(0)
    }
}
impl core::fmt::Debug for Irqpush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqpush")
            .field("irq", &self.irq())
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqpush {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqpush {{ irq: {=u8:?}, done: {=bool:?} }}",
            self.irq(),
            self.done()
        )
    }
}
#[doc = "Base0 and Base1 mapped offsets for ports"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mapbase(pub u32);
impl Mapbase {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn base0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_base0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn base1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_base1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mapbase {
    #[inline(always)]
    fn default() -> Mapbase {
        Mapbase(0)
    }
}
impl core::fmt::Debug for Mapbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mapbase")
            .field("base0", &self.base0())
            .field("base1", &self.base1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mapbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mapbase {{ base0: {=u16:?}, base1: {=u16:?} }}",
            self.base0(),
            self.base1()
        )
    }
}
#[doc = "Master Control for whole peripheral"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "The main enable for the whole block"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "The main enable for the whole block"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Port enable bits."]
    #[must_use]
    #[inline(always)]
    pub const fn pena(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Port enable bits."]
    #[inline(always)]
    pub const fn set_pena(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Port 80 enable."]
    #[must_use]
    #[inline(always)]
    pub const fn p80ena(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port 80 enable."]
    #[inline(always)]
    pub const fn set_p80ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status block is enabled and mapped according to the STATADDR register."]
    #[must_use]
    #[inline(always)]
    pub const fn sblkena(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status block is enabled and mapped according to the STATADDR register."]
    #[inline(always)]
    pub const fn set_sblkena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, then the functional clock provided to the block is 60MHz vs"]
    #[must_use]
    #[inline(always)]
    pub const fn use60mhz(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, then the functional clock provided to the block is 60MHz vs"]
    #[inline(always)]
    pub const fn set_use60mhz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("enable", &self.enable())
            .field("pena", &self.pena())
            .field("p80ena", &self.p80ena())
            .field("sblkena", &self.sblkena())
            .field("use60mhz", &self.use60mhz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ enable: {:?}, pena: {=u8:?}, p80ena: {=bool:?}, sblkena: {=bool:?}, use60mhz: {=bool:?} }}",
            self.enable(),
            self.pena(),
            self.p80ena(),
            self.sblkena(),
            self.use60mhz()
        )
    }
}
#[doc = "Master Status of whole peripheral"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstat(pub u32);
impl Mstat {
    #[doc = "Corresponding port is pending interrupt service"]
    #[must_use]
    #[inline(always)]
    pub const fn port_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Corresponding port is pending interrupt service"]
    #[inline(always)]
    pub const fn set_port_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 has had a request and is pending service."]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 has had a request and is pending service."]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If 1, the entered or left reset. Sticky - must clear."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_rst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the entered or left reset. Sticky - must clear."]
    #[inline(always)]
    pub const fn set_bus_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_upd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the bus had an IRQ update completion (for eSPI, IRQPush done; for LPC, SERIRQ done)"]
    #[inline(always)]
    pub const fn set_irq_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
    #[must_use]
    #[inline(always)]
    pub const fn wire_chg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, one or more input VWire has changed since last cleared for eSPI; for LPC, SERIRQ started"]
    #[inline(always)]
    pub const fn set_wire_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host is stalled on a read from or write to a port that has the StallRd or StallWr bit set in the PnCFG register"]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "If 1, the CRC from the Master did not match the computed CRC"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the CRC from the Master did not match the computed CRC"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "If 1, the GPIO in ESPIMISC has had an input change"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the GPIO in ESPIMISC has had an input change"]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "If 1, the bus is busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the bus is busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "If 1, the bus in reset."]
    #[must_use]
    #[inline(always)]
    pub const fn in_rst(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the bus in reset."]
    #[inline(always)]
    pub const fn set_in_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, completions are pending for eSPI; indicates quiet mode for LPC."]
    #[must_use]
    #[inline(always)]
    pub const fn comp_pend(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, completions are pending for eSPI; indicates quiet mode for LPC."]
    #[inline(always)]
    pub const fn set_comp_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "If 1, Mastering is pending (flash or memory)"]
    #[must_use]
    #[inline(always)]
    pub const fn mast_pend(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, Mastering is pending (flash or memory)"]
    #[inline(always)]
    pub const fn set_mast_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "If 1, the Alert request pin is pending (whether separate pin or MISO)"]
    #[must_use]
    #[inline(always)]
    pub const fn alert_pend(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Alert request pin is pending (whether separate pin or MISO)"]
    #[inline(always)]
    pub const fn set_alert_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Mstat {
    #[inline(always)]
    fn default() -> Mstat {
        Mstat(0)
    }
}
impl core::fmt::Debug for Mstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstat")
            .field("port_int", &self.port_int())
            .field("p80int", &self.p80int())
            .field("bus_rst", &self.bus_rst())
            .field("irq_upd", &self.irq_upd())
            .field("wire_chg", &self.wire_chg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("busy", &self.busy())
            .field("in_rst", &self.in_rst())
            .field("comp_pend", &self.comp_pend())
            .field("mast_pend", &self.mast_pend())
            .field("alert_pend", &self.alert_pend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstat {{ port_int: {=u8:?}, p80int: {=bool:?}, bus_rst: {=bool:?}, irq_upd: {=bool:?}, wire_chg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?}, busy: {=bool:?}, in_rst: {=bool:?}, comp_pend: {=bool:?}, mast_pend: {=bool:?}, alert_pend: {=bool:?} }}",
            self.port_int(),
            self.p80int(),
            self.bus_rst(),
            self.irq_upd(),
            self.wire_chg(),
            self.hstall(),
            self.crcerr(),
            self.gpio(),
            self.busy(),
            self.in_rst(),
            self.comp_pend(),
            self.mast_pend(),
            self.alert_pend()
        )
    }
}
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0addr(pub u32);
impl P0addr {
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_or_asz(&self) -> super::vals::P0addrBaseOrAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P0addrBaseOrAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_or_asz(&mut self, val: super::vals::P0addrBaseOrAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for P0addr {
    #[inline(always)]
    fn default() -> P0addr {
        P0addr(0)
    }
}
impl core::fmt::Debug for P0addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0addr")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0addr {{ off: {=u16:?}, base_or_asz: {:?}, idxoff: {=u8:?}, idx1st: {=bool:?} }}",
            self.off(),
            self.base_or_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0cfg(pub u32);
impl P0cfg {
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P0cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P0cfgType::from_bits(val as u8)
    }
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P0cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[must_use]
    #[inline(always)]
    pub const fn mbint_all(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub const fn set_mbint_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_rd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub const fn set_stall_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_wr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub const fn set_stall_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ign(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub const fn set_error_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for P0cfg {
    #[inline(always)]
    fn default() -> P0cfg {
        P0cfg(0)
    }
}
impl core::fmt::Debug for P0cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0cfg {{ type_: {:?}, direction: {=u8:?}, mbint_all: {=bool:?}, stall_rd: {=bool:?}, stall_wr: {=bool:?}, error_ign: {=bool:?} }}",
            self.type_(),
            self.direction(),
            self.mbint_all(),
            self.stall_rd(),
            self.stall_wr(),
            self.error_ign()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0dataIn(pub u32);
impl P0dataIn {
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of last access (ie"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of last access (ie"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P0dataIn {
    #[inline(always)]
    fn default() -> P0dataIn {
        P0dataIn(0)
    }
}
impl core::fmt::Debug for P0dataIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0dataIn")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0dataIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0dataIn {{ data_len: {=u8:?}, dir: {=bool:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0dataOut(pub u32);
impl P0dataOut {
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P0dataOut {
    #[inline(always)]
    fn default() -> P0dataOut {
        P0dataOut(0)
    }
}
impl core::fmt::Debug for P0dataOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0dataOut")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0dataOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P0dataOut {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0iruleStat(pub u32);
impl P0iruleStat {
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status set/clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status set/clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P0iruleStat {
    #[inline(always)]
    fn default() -> P0iruleStat {
        P0iruleStat(0)
    }
}
impl core::fmt::Debug for P0iruleStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0iruleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0iruleStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0iruleStat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst()
        )
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0omflen(pub u32);
impl P0omflen {
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for P0omflen {
    #[inline(always)]
    fn default() -> P0omflen {
        P0omflen(0)
    }
}
impl core::fmt::Debug for P0omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0omflen {{ len: {=u8:?}, trans: {=u8:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ramuse(pub u32);
impl P0ramuse {
    #[doc = "This is the word offset into the RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is the word offset into the RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P0ramuse {
    #[inline(always)]
    fn default() -> P0ramuse {
        P0ramuse(0)
    }
}
impl core::fmt::Debug for P0ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0stat(pub u32);
impl P0stat {
    #[doc = "Status of Host Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Read data"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt was caused by error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for P0stat {
    #[inline(always)]
    fn default() -> P0stat {
        P0stat(0)
    }
}
impl core::fmt::Debug for P0stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0stat {{ rdstat: {=u8:?}, wrstat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, err: {=u8:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.err()
        )
    }
}
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1addr(pub u32);
impl P1addr {
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_or_asz(&self) -> super::vals::P1addrBaseOrAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P1addrBaseOrAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_or_asz(&mut self, val: super::vals::P1addrBaseOrAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for P1addr {
    #[inline(always)]
    fn default() -> P1addr {
        P1addr(0)
    }
}
impl core::fmt::Debug for P1addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1addr")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1addr {{ off: {=u16:?}, base_or_asz: {:?}, idxoff: {=u8:?}, idx1st: {=bool:?} }}",
            self.off(),
            self.base_or_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1cfg(pub u32);
impl P1cfg {
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P1cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P1cfgType::from_bits(val as u8)
    }
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P1cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[must_use]
    #[inline(always)]
    pub const fn mbint_all(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub const fn set_mbint_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_rd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub const fn set_stall_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_wr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub const fn set_stall_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ign(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub const fn set_error_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for P1cfg {
    #[inline(always)]
    fn default() -> P1cfg {
        P1cfg(0)
    }
}
impl core::fmt::Debug for P1cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1cfg {{ type_: {:?}, direction: {=u8:?}, mbint_all: {=bool:?}, stall_rd: {=bool:?}, stall_wr: {=bool:?}, error_ign: {=bool:?} }}",
            self.type_(),
            self.direction(),
            self.mbint_all(),
            self.stall_rd(),
            self.stall_wr(),
            self.error_ign()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1dataIn(pub u32);
impl P1dataIn {
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of last access (ie"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of last access (ie"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P1dataIn {
    #[inline(always)]
    fn default() -> P1dataIn {
        P1dataIn(0)
    }
}
impl core::fmt::Debug for P1dataIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1dataIn")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1dataIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1dataIn {{ data_len: {=u8:?}, dir: {=bool:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1dataOut(pub u32);
impl P1dataOut {
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P1dataOut {
    #[inline(always)]
    fn default() -> P1dataOut {
        P1dataOut(0)
    }
}
impl core::fmt::Debug for P1dataOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1dataOut")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1dataOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P1dataOut {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1iruleStat(pub u32);
impl P1iruleStat {
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status set/clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status set/clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P1iruleStat {
    #[inline(always)]
    fn default() -> P1iruleStat {
        P1iruleStat(0)
    }
}
impl core::fmt::Debug for P1iruleStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1iruleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1iruleStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1iruleStat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst()
        )
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1omflen(pub u32);
impl P1omflen {
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for P1omflen {
    #[inline(always)]
    fn default() -> P1omflen {
        P1omflen(0)
    }
}
impl core::fmt::Debug for P1omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1omflen {{ len: {=u8:?}, trans: {=u8:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ramuse(pub u32);
impl P1ramuse {
    #[doc = "This is the word offset into the RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is the word offset into the RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P1ramuse {
    #[inline(always)]
    fn default() -> P1ramuse {
        P1ramuse(0)
    }
}
impl core::fmt::Debug for P1ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1stat(pub u32);
impl P1stat {
    #[doc = "Status of Host Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Read data"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt was caused by error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for P1stat {
    #[inline(always)]
    fn default() -> P1stat {
        P1stat(0)
    }
}
impl core::fmt::Debug for P1stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1stat {{ rdstat: {=u8:?}, wrstat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, err: {=u8:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.err()
        )
    }
}
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2addr(pub u32);
impl P2addr {
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_or_asz(&self) -> super::vals::P2addrBaseOrAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P2addrBaseOrAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_or_asz(&mut self, val: super::vals::P2addrBaseOrAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for P2addr {
    #[inline(always)]
    fn default() -> P2addr {
        P2addr(0)
    }
}
impl core::fmt::Debug for P2addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2addr")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2addr {{ off: {=u16:?}, base_or_asz: {:?}, idxoff: {=u8:?}, idx1st: {=bool:?} }}",
            self.off(),
            self.base_or_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2cfg(pub u32);
impl P2cfg {
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P2cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P2cfgType::from_bits(val as u8)
    }
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P2cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[must_use]
    #[inline(always)]
    pub const fn mbint_all(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub const fn set_mbint_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_rd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub const fn set_stall_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_wr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub const fn set_stall_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ign(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub const fn set_error_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for P2cfg {
    #[inline(always)]
    fn default() -> P2cfg {
        P2cfg(0)
    }
}
impl core::fmt::Debug for P2cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2cfg {{ type_: {:?}, direction: {=u8:?}, mbint_all: {=bool:?}, stall_rd: {=bool:?}, stall_wr: {=bool:?}, error_ign: {=bool:?} }}",
            self.type_(),
            self.direction(),
            self.mbint_all(),
            self.stall_rd(),
            self.stall_wr(),
            self.error_ign()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2dataIn(pub u32);
impl P2dataIn {
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of last access (ie"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of last access (ie"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P2dataIn {
    #[inline(always)]
    fn default() -> P2dataIn {
        P2dataIn(0)
    }
}
impl core::fmt::Debug for P2dataIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2dataIn")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2dataIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2dataIn {{ data_len: {=u8:?}, dir: {=bool:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2dataOut(pub u32);
impl P2dataOut {
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P2dataOut {
    #[inline(always)]
    fn default() -> P2dataOut {
        P2dataOut(0)
    }
}
impl core::fmt::Debug for P2dataOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2dataOut")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2dataOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P2dataOut {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2iruleStat(pub u32);
impl P2iruleStat {
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status set/clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status set/clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P2iruleStat {
    #[inline(always)]
    fn default() -> P2iruleStat {
        P2iruleStat(0)
    }
}
impl core::fmt::Debug for P2iruleStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2iruleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2iruleStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2iruleStat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst()
        )
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2omflen(pub u32);
impl P2omflen {
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for P2omflen {
    #[inline(always)]
    fn default() -> P2omflen {
        P2omflen(0)
    }
}
impl core::fmt::Debug for P2omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2omflen {{ len: {=u8:?}, trans: {=u8:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ramuse(pub u32);
impl P2ramuse {
    #[doc = "This is the word offset into the RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is the word offset into the RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P2ramuse {
    #[inline(always)]
    fn default() -> P2ramuse {
        P2ramuse(0)
    }
}
impl core::fmt::Debug for P2ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2stat(pub u32);
impl P2stat {
    #[doc = "Status of Host Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Read data"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt was caused by error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for P2stat {
    #[inline(always)]
    fn default() -> P2stat {
        P2stat(0)
    }
}
impl core::fmt::Debug for P2stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2stat {{ rdstat: {=u8:?}, wrstat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, err: {=u8:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.err()
        )
    }
}
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3addr(pub u32);
impl P3addr {
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_or_asz(&self) -> super::vals::P3addrBaseOrAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P3addrBaseOrAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_or_asz(&mut self, val: super::vals::P3addrBaseOrAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for P3addr {
    #[inline(always)]
    fn default() -> P3addr {
        P3addr(0)
    }
}
impl core::fmt::Debug for P3addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3addr")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3addr {{ off: {=u16:?}, base_or_asz: {:?}, idxoff: {=u8:?}, idx1st: {=bool:?} }}",
            self.off(),
            self.base_or_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3cfg(pub u32);
impl P3cfg {
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P3cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P3cfgType::from_bits(val as u8)
    }
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P3cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[must_use]
    #[inline(always)]
    pub const fn mbint_all(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub const fn set_mbint_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_rd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub const fn set_stall_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_wr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub const fn set_stall_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ign(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub const fn set_error_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for P3cfg {
    #[inline(always)]
    fn default() -> P3cfg {
        P3cfg(0)
    }
}
impl core::fmt::Debug for P3cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3cfg {{ type_: {:?}, direction: {=u8:?}, mbint_all: {=bool:?}, stall_rd: {=bool:?}, stall_wr: {=bool:?}, error_ign: {=bool:?} }}",
            self.type_(),
            self.direction(),
            self.mbint_all(),
            self.stall_rd(),
            self.stall_wr(),
            self.error_ign()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3dataIn(pub u32);
impl P3dataIn {
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of last access (ie"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of last access (ie"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P3dataIn {
    #[inline(always)]
    fn default() -> P3dataIn {
        P3dataIn(0)
    }
}
impl core::fmt::Debug for P3dataIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3dataIn")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3dataIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3dataIn {{ data_len: {=u8:?}, dir: {=bool:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3dataOut(pub u32);
impl P3dataOut {
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P3dataOut {
    #[inline(always)]
    fn default() -> P3dataOut {
        P3dataOut(0)
    }
}
impl core::fmt::Debug for P3dataOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3dataOut")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3dataOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P3dataOut {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3iruleStat(pub u32);
impl P3iruleStat {
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status set/clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status set/clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P3iruleStat {
    #[inline(always)]
    fn default() -> P3iruleStat {
        P3iruleStat(0)
    }
}
impl core::fmt::Debug for P3iruleStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3iruleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3iruleStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3iruleStat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst()
        )
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3omflen(pub u32);
impl P3omflen {
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for P3omflen {
    #[inline(always)]
    fn default() -> P3omflen {
        P3omflen(0)
    }
}
impl core::fmt::Debug for P3omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3omflen {{ len: {=u8:?}, trans: {=u8:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3ramuse(pub u32);
impl P3ramuse {
    #[doc = "This is the word offset into the RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is the word offset into the RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P3ramuse {
    #[inline(always)]
    fn default() -> P3ramuse {
        P3ramuse(0)
    }
}
impl core::fmt::Debug for P3ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3stat(pub u32);
impl P3stat {
    #[doc = "Status of Host Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Read data"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt was caused by error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for P3stat {
    #[inline(always)]
    fn default() -> P3stat {
        P3stat(0)
    }
}
impl core::fmt::Debug for P3stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3stat {{ rdstat: {=u8:?}, wrstat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, err: {=u8:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.err()
        )
    }
}
#[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4addr(pub u32);
impl P4addr {
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset from 0 or the selected mapped base for matching memory or IO"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_or_asz(&self) -> super::vals::P4addrBaseOrAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P4addrBaseOrAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_or_asz(&mut self, val: super::vals::P4addrBaseOrAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "For index/register only: This is the byte offset of the Index relative to the data (before or after, based on IDX1ST)"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "For index/data register only: Is 1 if index is lower address than data (e"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for P4addr {
    #[inline(always)]
    fn default() -> P4addr {
        P4addr(0)
    }
}
impl core::fmt::Debug for P4addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4addr")
            .field("off", &self.off())
            .field("base_or_asz", &self.base_or_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4addr {{ off: {=u16:?}, base_or_asz: {:?}, idxoff: {=u8:?}, idx1st: {=bool:?} }}",
            self.off(),
            self.base_or_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4cfg(pub u32);
impl P4cfg {
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P4cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P4cfgType::from_bits(val as u8)
    }
    #[doc = "The Type field selects how the port interacts with the Host over the eSPI or LPC bus"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P4cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Meaning depends on Type, but generally picks the direction of the port (Host writes, Host reads, or both)"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[must_use]
    #[inline(always)]
    pub const fn mbint_all(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: Map interrupt on Read 1st and Write 1st to every read/write (special will still control 0th and last location)"]
    #[inline(always)]
    pub const fn set_mbint_all(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_rd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Read of Index/Data and Mailbox (only)"]
    #[inline(always)]
    pub const fn set_stall_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_wr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any Write of Index/Data and Mailbox"]
    #[inline(always)]
    pub const fn set_stall_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ign(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the Host will get an error when trying to perform a read or write that is blocked by the Direction field"]
    #[inline(always)]
    pub const fn set_error_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for P4cfg {
    #[inline(always)]
    fn default() -> P4cfg {
        P4cfg(0)
    }
}
impl core::fmt::Debug for P4cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbint_all", &self.mbint_all())
            .field("stall_rd", &self.stall_rd())
            .field("stall_wr", &self.stall_wr())
            .field("error_ign", &self.error_ign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4cfg {{ type_: {:?}, direction: {=u8:?}, mbint_all: {=bool:?}, stall_rd: {=bool:?}, stall_wr: {=bool:?}, error_ign: {=bool:?} }}",
            self.type_(),
            self.direction(),
            self.mbint_all(),
            self.stall_rd(),
            self.stall_wr(),
            self.error_ign()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4dataIn(pub u32);
impl P4dataIn {
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Meaning is dependent on type of port: If endpoint or index/data: is data byte If Mailbox or Bus master or Flash: is count in last message, 0 relative If SAF, is count in last message including address (so, +4), 0 relative"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of last access: 0 = Read By Host 1 = Write By Host"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of last access (ie"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of last access (ie"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P4dataIn {
    #[inline(always)]
    fn default() -> P4dataIn {
        P4dataIn(0)
    }
}
impl core::fmt::Debug for P4dataIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4dataIn")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4dataIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4dataIn {{ data_len: {=u8:?}, dir: {=bool:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4dataOut(pub u32);
impl P4dataOut {
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to send to Host. Application can tell when taken via the PnStatus register."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P4dataOut {
    #[inline(always)]
    fn default() -> P4dataOut {
        P4dataOut(0)
    }
}
impl core::fmt::Debug for P4dataOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4dataOut")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4dataOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P4dataOut {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4iruleStat(pub u32);
impl P4iruleStat {
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User defined status bits. These are salted into the status register as specified in section 4.1 and 4.6."]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if an error is detected (classes of error defined in section 2.14)"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt if Write or 1st Write or Bus master Finished."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt if: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status set/clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status set/clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the RdStatus and WrStatus in PStatus register"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P4iruleStat {
    #[inline(always)]
    fn default() -> P4iruleStat {
        P4iruleStat(0)
    }
}
impl core::fmt::Debug for P4iruleStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4iruleStat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4iruleStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4iruleStat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst()
        )
    }
}
#[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4omflen(pub u32);
impl P4omflen {
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in bytes, 0 relative, to Send for OOB, Send or Retrieve for Mastering, Read or Write for Flash, Erase (in blocks) for Flash, or read data for SAF read"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Transfer request as: Type Value Meaning OOB 0 To Host OOB 1, 2, 3 Not used Master 0 To Host 32 (Host reads w/32-bit address) Master 1 To Host 64 (Host reads w/64-bit address) Master 2 From Host 32 (Host writes w/32-bit address) Master 3 From Host 64 (Host writes w/64-bit address) MAF Flash 1 Read Flash (location in RAM) MAF Flash 2 Write Flash (location in RAM) MAF Flash 3 Erase Flash (sector in RAM) SAF 0 Completion fail SAF 1 Completion with data SAF 2 Completion with no data"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for P4omflen {
    #[inline(always)]
    fn default() -> P4omflen {
        P4omflen(0)
    }
}
impl core::fmt::Debug for P4omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4omflen {{ len: {=u8:?}, trans: {=u8:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4ramuse(pub u32);
impl P4ramuse {
    #[doc = "This is the word offset into the RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is the word offset into the RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This is the length of the mailbox or mastering area as 4<<LEN per direction"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P4ramuse {
    #[inline(always)]
    fn default() -> P4ramuse {
        P4ramuse(0)
    }
}
impl core::fmt::Debug for P4ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4stat(pub u32);
impl P4stat {
    #[doc = "Status of Host Read data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Read data"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt was caused by error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Read or 1st Read or Bus master Started."]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt was caused by Write or 1st Write or Bus master Started."]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt was caused by: Bit Endpoint Idx/Data Mbox BusMaster/Flash SPC0 CMD Idx Change Write 0 Completed SPC1 - - Read 0 - SPC2 - - Write Last - SPC3 - - Read Last -"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Cause of INTERR: Bit Endpoint, Idx/Data Mbox BusMaster/Flash ERR0 Host Write Over Host Write or Read invalid for Access Failed From-Host (cannot tell on Memory To-Host, but can for Flash write)"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for P4stat {
    #[inline(always)]
    fn default() -> P4stat {
        P4stat(0)
    }
}
impl core::fmt::Debug for P4stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4stat {{ rdstat: {=u8:?}, wrstat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, err: {=u8:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.err()
        )
    }
}
#[doc = "Port 80 Status (byte and prev byte)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80stat(pub u32);
impl P80stat {
    #[doc = "Has the Current Port80 value."]
    #[must_use]
    #[inline(always)]
    pub const fn curr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Has the Current Port80 value."]
    #[inline(always)]
    pub const fn set_curr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Has the previous Port80 value."]
    #[must_use]
    #[inline(always)]
    pub const fn prev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Has the previous Port80 value."]
    #[inline(always)]
    pub const fn set_prev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "A counter (set to 0 on enable of p80) which increments with each new value. Will wrap back to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "A counter (set to 0 on enable of p80) which increments with each new value. Will wrap back to 0."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Resets counter, CNT, back to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Resets counter, CNT, back to 0."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P80stat {
    #[inline(always)]
    fn default() -> P80stat {
        P80stat(0)
    }
}
impl core::fmt::Debug for P80stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P80stat")
            .field("curr", &self.curr())
            .field("prev", &self.prev())
            .field("cnt", &self.cnt())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P80stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P80stat {{ curr: {=u8:?}, prev: {=u8:?}, cnt: {=u8:?}, rst: {=bool:?} }}",
            self.curr(),
            self.prev(),
            self.cnt(),
            self.rst()
        )
    }
}
#[doc = "Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rambase(pub u32);
impl Rambase {
    #[doc = "Always 0"]
    #[must_use]
    #[inline(always)]
    pub const fn zero(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Always 0"]
    #[inline(always)]
    pub const fn set_zero(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
    #[must_use]
    #[inline(always)]
    pub const fn ram(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Is location in System memory space where RAM is located that is used by this peripheral's bus master"]
    #[inline(always)]
    pub const fn set_ram(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Rambase {
    #[inline(always)]
    fn default() -> Rambase {
        Rambase(0)
    }
}
impl core::fmt::Debug for Rambase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rambase")
            .field("zero", &self.zero())
            .field("ram", &self.ram())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rambase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rambase {{ zero: {=u16:?}, ram: {=u32:?} }}",
            self.zero(),
            self.ram()
        )
    }
}
#[doc = "Location of Status block in memory space, if enabled."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stataddr(pub u32);
impl Stataddr {
    #[doc = "The offset in Host space for the status block. It must be double-word aligned."]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "The offset in Host space for the status block. It must be double-word aligned."]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> super::vals::Base {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Base::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: super::vals::Base) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for Stataddr {
    #[inline(always)]
    fn default() -> Stataddr {
        Stataddr(0)
    }
}
impl core::fmt::Debug for Stataddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stataddr")
            .field("off", &self.off())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stataddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stataddr {{ off: {=u16:?}, base: {:?} }}",
            self.off(),
            self.base()
        )
    }
}
#[doc = "Wire states from Host"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wirero(pub u32);
impl Wirero {
    #[doc = "eSPI: Indicates the latest states from the Host via VWire"]
    #[must_use]
    #[inline(always)]
    pub const fn wires(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "eSPI: Indicates the latest states from the Host via VWire"]
    #[inline(always)]
    pub const fn set_wires(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Wirero {
    #[inline(always)]
    fn default() -> Wirero {
        Wirero(0)
    }
}
impl core::fmt::Debug for Wirero {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wirero")
            .field("wires", &self.wires())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wirero {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wirero {{ wires: {=u32:?} }}", self.wires())
    }
}
#[doc = "Wire states for Host to see; if LPC, this is the IRQ states."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wirewo(pub u32);
impl Wirewo {
    #[doc = "LPC: IRQ states to use"]
    #[must_use]
    #[inline(always)]
    pub const fn wires(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "LPC: IRQ states to use"]
    #[inline(always)]
    pub const fn set_wires(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Set to 1 after last write has been pushed out to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 after last write has been pushed out to Host"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Wirewo {
    #[inline(always)]
    fn default() -> Wirewo {
        Wirewo(0)
    }
}
impl core::fmt::Debug for Wirewo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wirewo")
            .field("wires", &self.wires())
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wirewo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wirewo {{ wires: {=u32:?}, done: {=bool:?} }}",
            self.wires(),
            self.done()
        )
    }
}
