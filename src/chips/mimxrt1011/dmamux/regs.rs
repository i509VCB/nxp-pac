#[doc = "Channel index Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chcfg(pub u32);
impl Chcfg {
    #[doc = "DMA Channel Source (Slot Number)"]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "DMA Channel Always Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn a_on(&self) -> super::vals::AOn {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::AOn::from_bits(val as u8)
    }
    #[doc = "DMA Channel Always Enable"]
    #[inline(always)]
    pub const fn set_a_on(&mut self, val: super::vals::AOn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA Channel Trigger Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trig(&self) -> super::vals::Trig {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Trig::from_bits(val as u8)
    }
    #[doc = "DMA Channel Trigger Enable"]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: super::vals::Trig) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Mux Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enbl(&self) -> super::vals::Enbl {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Enbl::from_bits(val as u8)
    }
    #[doc = "DMA Mux Channel Enable"]
    #[inline(always)]
    pub const fn set_enbl(&mut self, val: super::vals::Enbl) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Chcfg {
    #[inline(always)]
    fn default() -> Chcfg {
        Chcfg(0)
    }
}
impl core::fmt::Debug for Chcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chcfg")
            .field("source", &self.source())
            .field("a_on", &self.a_on())
            .field("trig", &self.trig())
            .field("enbl", &self.enbl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Chcfg {{ source: {=u8:?}, a_on: {:?}, trig: {:?}, enbl: {:?} }}",
            self.source(),
            self.a_on(),
            self.trig(),
            self.enbl()
        )
    }
}
