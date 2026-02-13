#[doc = "Data Remap"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "Remap Lock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn remaplk(&self) -> super::vals::Remaplk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Remaplk::from_bits(val as u8)
    }
    #[doc = "Remap Lock Enable"]
    #[inline(always)]
    pub const fn set_remaplk(&mut self, val: super::vals::Remaplk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LIM Remapping Address"]
    #[must_use]
    #[inline(always)]
    pub const fn lim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "LIM Remapping Address"]
    #[inline(always)]
    pub const fn set_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "LIMDP Remapping Address"]
    #[must_use]
    #[inline(always)]
    pub const fn limdp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "LIMDP Remapping Address"]
    #[inline(always)]
    pub const fn set_limdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("remaplk", &self.remaplk())
            .field("lim", &self.lim())
            .field("limdp", &self.limdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ remaplk: {:?}, lim: {=u8:?}, limdp: {=u8:?} }}",
            self.remaplk(),
            self.lim(),
            self.limdp()
        )
    }
}
