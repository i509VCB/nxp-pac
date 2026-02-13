#[doc = "AHB Bus Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbcr(pub u32);
impl Ahbcr {
    #[doc = "Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[must_use]
    #[inline(always)]
    pub const fn aparen(&self) -> super::vals::Aparen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aparen::from_bits(val as u8)
    }
    #[doc = "Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub const fn set_aparen(&mut self, val: super::vals::Aparen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable AHB bus cachable read access support."]
    #[must_use]
    #[inline(always)]
    pub const fn cachableen(&self) -> super::vals::Cachableen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cachableen::from_bits(val as u8)
    }
    #[doc = "Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub const fn set_cachableen(&mut self, val: super::vals::Cachableen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[must_use]
    #[inline(always)]
    pub const fn bufferableen(&self) -> super::vals::Bufferableen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bufferableen::from_bits(val as u8)
    }
    #[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub const fn set_bufferableen(&mut self, val: super::vals::Bufferableen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[must_use]
    #[inline(always)]
    pub const fn readaddropt(&self) -> super::vals::Readaddropt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Readaddropt::from_bits(val as u8)
    }
    #[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub const fn set_readaddropt(&mut self, val: super::vals::Readaddropt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Ahbcr {
    #[inline(always)]
    fn default() -> Ahbcr {
        Ahbcr(0)
    }
}
impl core::fmt::Debug for Ahbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbcr")
            .field("aparen", &self.aparen())
            .field("cachableen", &self.cachableen())
            .field("bufferableen", &self.bufferableen())
            .field("prefetchen", &self.prefetchen())
            .field("readaddropt", &self.readaddropt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbcr {{ aparen: {:?}, cachableen: {:?}, bufferableen: {:?}, prefetchen: {=bool:?}, readaddropt: {:?} }}",
            self.aparen(),
            self.cachableen(),
            self.bufferableen(),
            self.prefetchen(),
            self.readaddropt()
        )
    }
}
#[doc = "AHB RX Buffer 0 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf0cr0(pub u32);
impl Ahbrxbuf0cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf0cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf0cr0 {
        Ahbrxbuf0cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf0cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf0cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf0cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf0cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 1 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf1cr0(pub u32);
impl Ahbrxbuf1cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf1cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf1cr0 {
        Ahbrxbuf1cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf1cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf1cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 2 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf2cr0(pub u32);
impl Ahbrxbuf2cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf2cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf2cr0 {
        Ahbrxbuf2cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf2cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf2cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 3 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf3cr0(pub u32);
impl Ahbrxbuf3cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf3cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf3cr0 {
        Ahbrxbuf3cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf3cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf3cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf3cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf3cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 4 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf4cr0(pub u32);
impl Ahbrxbuf4cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf4cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf4cr0 {
        Ahbrxbuf4cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf4cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf4cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf4cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf4cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 5 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf5cr0(pub u32);
impl Ahbrxbuf5cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf5cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf5cr0 {
        Ahbrxbuf5cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf5cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf5cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf5cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf5cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 6 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf6cr0(pub u32);
impl Ahbrxbuf6cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf6cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf6cr0 {
        Ahbrxbuf6cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf6cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf6cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf6cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf6cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB RX Buffer 7 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf7cr0(pub u32);
impl Ahbrxbuf7cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf7cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf7cr0 {
        Ahbrxbuf7cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf7cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf7cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf7cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf7cr0 {{ bufsz: {=u16:?}, mstrid: {=u8:?}, priority: {=u8:?}, prefetchen: {=bool:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Suspend Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbspndsts(pub u32);
impl Ahbspndsts {
    #[doc = "Indicates if an AHB read prefetch command sequence has been suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB RX BUF ID for suspended command sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn bufid(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub const fn set_bufid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Left Data size for suspended command sequence (in byte)."]
    #[must_use]
    #[inline(always)]
    pub const fn datlft(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Left Data size for suspended command sequence (in byte)."]
    #[inline(always)]
    pub const fn set_datlft(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ahbspndsts {
    #[inline(always)]
    fn default() -> Ahbspndsts {
        Ahbspndsts(0)
    }
}
impl core::fmt::Debug for Ahbspndsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbspndsts")
            .field("active", &self.active())
            .field("bufid", &self.bufid())
            .field("datlft", &self.datlft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbspndsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbspndsts {{ active: {=bool:?}, bufid: {=u8:?}, datlft: {=u16:?} }}",
            self.active(),
            self.bufid(),
            self.datlft()
        )
    }
}
#[doc = "DLL Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL calibration enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL calibration enable."]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Slave clock delay line delay cell number selection override enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave clock delay line delay cell number selection override value."]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
}
impl Default for Dllcr {
    #[inline(always)]
    fn default() -> Dllcr {
        Dllcr(0)
    }
}
impl core::fmt::Debug for Dllcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dllcr")
            .field("dllen", &self.dllen())
            .field("dllreset", &self.dllreset())
            .field("slvdlytarget", &self.slvdlytarget())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dllcr {{ dllen: {=bool:?}, dllreset: {=bool:?}, slvdlytarget: {=u8:?}, ovrden: {=bool:?}, ovrdval: {=u8:?} }}",
            self.dllen(),
            self.dllreset(),
            self.slvdlytarget(),
            self.ovrden(),
            self.ovrdval()
        )
    }
}
#[doc = "Data Learn Pattern Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlpr(pub u32);
impl Dlpr {
    #[doc = "Data Learning Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn dlp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Learning Pattern."]
    #[inline(always)]
    pub const fn set_dlp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dlpr {
    #[inline(always)]
    fn default() -> Dlpr {
        Dlpr(0)
    }
}
impl core::fmt::Debug for Dlpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dlpr").field("dlp", &self.dlp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dlpr {{ dlp: {=u32:?} }}", self.dlp())
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha1cr0(pub u32);
impl Flsha1cr0 {
    #[doc = "Flash Size in KByte."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flsha1cr0 {
    #[inline(always)]
    fn default() -> Flsha1cr0 {
        Flsha1cr0(0)
    }
}
impl core::fmt::Debug for Flsha1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha1cr0")
            .field("flshsz", &self.flshsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flsha1cr0 {{ flshsz: {=u32:?} }}", self.flshsz())
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha2cr0(pub u32);
impl Flsha2cr0 {
    #[doc = "Flash Size in KByte."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flsha2cr0 {
    #[inline(always)]
    fn default() -> Flsha2cr0 {
        Flsha2cr0(0)
    }
}
impl core::fmt::Debug for Flsha2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha2cr0")
            .field("flshsz", &self.flshsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flsha2cr0 {{ flshsz: {=u32:?} }}", self.flshsz())
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb1cr0(pub u32);
impl Flshb1cr0 {
    #[doc = "Flash Size in KByte."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flshb1cr0 {
    #[inline(always)]
    fn default() -> Flshb1cr0 {
        Flshb1cr0(0)
    }
}
impl core::fmt::Debug for Flshb1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb1cr0")
            .field("flshsz", &self.flshsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flshb1cr0 {{ flshsz: {=u32:?} }}", self.flshsz())
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb2cr0(pub u32);
impl Flshb2cr0 {
    #[doc = "Flash Size in KByte."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flshb2cr0 {
    #[inline(always)]
    fn default() -> Flshb2cr0 {
        Flshb2cr0(0)
    }
}
impl core::fmt::Debug for Flshb2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb2cr0")
            .field("flshsz", &self.flshsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flshb2cr0 {{ flshsz: {=u32:?} }}", self.flshsz())
    }
}
#[doc = "Flash Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr1(pub u32);
impl Flshcr1 {
    #[doc = "Serial Flash CS setup time."]
    #[must_use]
    #[inline(always)]
    pub const fn tcss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS setup time."]
    #[inline(always)]
    pub const fn set_tcss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Hold time."]
    #[must_use]
    #[inline(always)]
    pub const fn tcsh(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Hold time."]
    #[inline(always)]
    pub const fn set_tcsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Word Addressable."]
    #[must_use]
    #[inline(always)]
    pub const fn wa(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Word Addressable."]
    #[inline(always)]
    pub const fn set_wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size."]
    #[must_use]
    #[inline(always)]
    pub const fn cas(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size."]
    #[inline(always)]
    pub const fn set_cas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "CS interval unit"]
    #[must_use]
    #[inline(always)]
    pub const fn csintervalunit(&self) -> super::vals::Csintervalunit {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Csintervalunit::from_bits(val as u8)
    }
    #[doc = "CS interval unit"]
    #[inline(always)]
    pub const fn set_csintervalunit(&mut self, val: super::vals::Csintervalunit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[must_use]
    #[inline(always)]
    pub const fn csinterval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub const fn set_csinterval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Flshcr1 {
    #[inline(always)]
    fn default() -> Flshcr1 {
        Flshcr1(0)
    }
}
impl core::fmt::Debug for Flshcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr1")
            .field("tcss", &self.tcss())
            .field("tcsh", &self.tcsh())
            .field("wa", &self.wa())
            .field("cas", &self.cas())
            .field("csintervalunit", &self.csintervalunit())
            .field("csinterval", &self.csinterval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr1 {{ tcss: {=u8:?}, tcsh: {=u8:?}, wa: {=bool:?}, cas: {=u8:?}, csintervalunit: {:?}, csinterval: {=u16:?} }}",
            self.tcss(),
            self.tcsh(),
            self.wa(),
            self.cas(),
            self.csintervalunit(),
            self.csinterval()
        )
    }
}
#[doc = "Flash Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr2(pub u32);
impl Flshcr2 {
    #[doc = "Sequence Index for AHB Read triggered Command in LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub const fn set_ardseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Sequence Number for AHB Read triggered Command in LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqnum(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub const fn set_ardseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Sequence Index for AHB Write triggered Command."]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub const fn set_awrseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Sequence Number for AHB Write triggered Command."]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub const fn set_awrseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub const fn set_awrwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AWRWAIT unit"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwaitunit(&self) -> super::vals::Awrwaitunit {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Awrwaitunit::from_bits(val as u8)
    }
    #[doc = "AWRWAIT unit"]
    #[inline(always)]
    pub const fn set_awrwaitunit(&mut self, val: super::vals::Awrwaitunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[must_use]
    #[inline(always)]
    pub const fn clrinstrptr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub const fn set_clrinstrptr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Flshcr2 {
    #[inline(always)]
    fn default() -> Flshcr2 {
        Flshcr2(0)
    }
}
impl core::fmt::Debug for Flshcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr2")
            .field("ardseqid", &self.ardseqid())
            .field("ardseqnum", &self.ardseqnum())
            .field("awrseqid", &self.awrseqid())
            .field("awrseqnum", &self.awrseqnum())
            .field("awrwait", &self.awrwait())
            .field("awrwaitunit", &self.awrwaitunit())
            .field("clrinstrptr", &self.clrinstrptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr2 {{ ardseqid: {=u8:?}, ardseqnum: {=u8:?}, awrseqid: {=u8:?}, awrseqnum: {=u8:?}, awrwait: {=u16:?}, awrwaitunit: {:?}, clrinstrptr: {=bool:?} }}",
            self.ardseqid(),
            self.ardseqnum(),
            self.awrseqid(),
            self.awrseqnum(),
            self.awrwait(),
            self.awrwaitunit(),
            self.clrinstrptr()
        )
    }
}
#[doc = "Flash Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr4(pub u32);
impl Flshcr4 {
    #[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt1(&self) -> super::vals::Wmopt1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wmopt1::from_bits(val as u8)
    }
    #[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub const fn set_wmopt1(&mut self, val: super::vals::Wmopt1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[must_use]
    #[inline(always)]
    pub const fn wmena(&self) -> super::vals::Wmena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wmena::from_bits(val as u8)
    }
    #[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub const fn set_wmena(&mut self, val: super::vals::Wmena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[must_use]
    #[inline(always)]
    pub const fn wmenb(&self) -> super::vals::Wmenb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wmenb::from_bits(val as u8)
    }
    #[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub const fn set_wmenb(&mut self, val: super::vals::Wmenb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Flshcr4 {
    #[inline(always)]
    fn default() -> Flshcr4 {
        Flshcr4(0)
    }
}
impl core::fmt::Debug for Flshcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr4")
            .field("wmopt1", &self.wmopt1())
            .field("wmena", &self.wmena())
            .field("wmenb", &self.wmenb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr4 {{ wmopt1: {:?}, wmena: {:?}, wmenb: {:?} }}",
            self.wmopt1(),
            self.wmena(),
            self.wmenb()
        )
    }
}
#[doc = "HADDR REMAP END ADDR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddrend(pub u32);
impl Haddrend {
    #[doc = "HADDR remap range's end addr, 4K aligned"]
    #[must_use]
    #[inline(always)]
    pub const fn endstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR remap range's end addr, 4K aligned"]
    #[inline(always)]
    pub const fn set_endstart(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddrend {
    #[inline(always)]
    fn default() -> Haddrend {
        Haddrend(0)
    }
}
impl core::fmt::Debug for Haddrend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddrend")
            .field("endstart", &self.endstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddrend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Haddrend {{ endstart: {=u32:?} }}", self.endstart())
    }
}
#[doc = "HADDR REMAP OFFSET"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddroffset(pub u32);
impl Haddroffset {
    #[doc = "HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
    #[must_use]
    #[inline(always)]
    pub const fn addroffset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR offset field, remapped address will be ADDR\\[31:12\\]=ADDR_original\\[31:12\\]+ADDROFFSET"]
    #[inline(always)]
    pub const fn set_addroffset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddroffset {
    #[inline(always)]
    fn default() -> Haddroffset {
        Haddroffset(0)
    }
}
impl core::fmt::Debug for Haddroffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddroffset")
            .field("addroffset", &self.addroffset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddroffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Haddroffset {{ addroffset: {=u32:?} }}",
            self.addroffset()
        )
    }
}
#[doc = "HADDR REMAP START ADDR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddrstart(pub u32);
impl Haddrstart {
    #[doc = "AHB Bus address remap function enable"]
    #[must_use]
    #[inline(always)]
    pub const fn remapen(&self) -> super::vals::Remapen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Remapen::from_bits(val as u8)
    }
    #[doc = "AHB Bus address remap function enable"]
    #[inline(always)]
    pub const fn set_remapen(&mut self, val: super::vals::Remapen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HADDR remap range's start addr, 4K aligned"]
    #[must_use]
    #[inline(always)]
    pub const fn addrstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR remap range's start addr, 4K aligned"]
    #[inline(always)]
    pub const fn set_addrstart(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddrstart {
    #[inline(always)]
    fn default() -> Haddrstart {
        Haddrstart(0)
    }
}
impl core::fmt::Debug for Haddrstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddrstart")
            .field("remapen", &self.remapen())
            .field("addrstart", &self.addrstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddrstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Haddrstart {{ remapen: {:?}, addrstart: {=u32:?} }}",
            self.remapen(),
            self.addrstart()
        )
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP triggered Command Sequences Execution finished interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdgeen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub const fn set_ipcmdgeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdgeen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub const fn set_ahbcmdgeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderren(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub const fn set_ahbcmderren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP RX FIFO WaterMark available interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwaen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub const fn set_iprxwaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP TX FIFO WaterMark empty interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iptxween(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub const fn set_iptxween(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning failed interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfailen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning failed interrupt enable."]
    #[inline(always)]
    pub const fn set_datalearnfailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrden(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub const fn set_sckstopbyrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywren(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub const fn set_sckstopbywren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeouten(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn set_ahbbustimeouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeouten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn set_seqtimeouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("ipcmddoneen", &self.ipcmddoneen())
            .field("ipcmdgeen", &self.ipcmdgeen())
            .field("ahbcmdgeen", &self.ahbcmdgeen())
            .field("ipcmderren", &self.ipcmderren())
            .field("ahbcmderren", &self.ahbcmderren())
            .field("iprxwaen", &self.iprxwaen())
            .field("iptxween", &self.iptxween())
            .field("datalearnfailen", &self.datalearnfailen())
            .field("sckstopbyrden", &self.sckstopbyrden())
            .field("sckstopbywren", &self.sckstopbywren())
            .field("ahbbustimeouten", &self.ahbbustimeouten())
            .field("seqtimeouten", &self.seqtimeouten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ipcmddoneen: {=bool:?}, ipcmdgeen: {=bool:?}, ahbcmdgeen: {=bool:?}, ipcmderren: {=bool:?}, ahbcmderren: {=bool:?}, iprxwaen: {=bool:?}, iptxween: {=bool:?}, datalearnfailen: {=bool:?}, sckstopbyrden: {=bool:?}, sckstopbywren: {=bool:?}, ahbbustimeouten: {=bool:?}, seqtimeouten: {=bool:?} }}",
            self.ipcmddoneen(),
            self.ipcmdgeen(),
            self.ahbcmdgeen(),
            self.ipcmderren(),
            self.ahbcmderren(),
            self.iprxwaen(),
            self.iptxween(),
            self.datalearnfailen(),
            self.sckstopbyrden(),
            self.sckstopbywren(),
            self.ahbbustimeouten(),
            self.seqtimeouten()
        )
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdge(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub const fn set_ipcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub const fn set_ahbcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub const fn set_ahbcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP RX FIFO watermark available interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub const fn set_iprxwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP TX FIFO watermark empty interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iptxwe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub const fn set_iptxwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning failed interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning failed interrupt."]
    #[inline(always)]
    pub const fn set_datalearnfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub const fn set_sckstopbyrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub const fn set_sckstopbywr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeout(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn set_ahbbustimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution timeout interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeout(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence execution timeout interrupt."]
    #[inline(always)]
    pub const fn set_seqtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("ipcmddone", &self.ipcmddone())
            .field("ipcmdge", &self.ipcmdge())
            .field("ahbcmdge", &self.ahbcmdge())
            .field("ipcmderr", &self.ipcmderr())
            .field("ahbcmderr", &self.ahbcmderr())
            .field("iprxwa", &self.iprxwa())
            .field("iptxwe", &self.iptxwe())
            .field("datalearnfail", &self.datalearnfail())
            .field("sckstopbyrd", &self.sckstopbyrd())
            .field("sckstopbywr", &self.sckstopbywr())
            .field("ahbbustimeout", &self.ahbbustimeout())
            .field("seqtimeout", &self.seqtimeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intr {{ ipcmddone: {=bool:?}, ipcmdge: {=bool:?}, ahbcmdge: {=bool:?}, ipcmderr: {=bool:?}, ahbcmderr: {=bool:?}, iprxwa: {=bool:?}, iptxwe: {=bool:?}, datalearnfail: {=bool:?}, sckstopbyrd: {=bool:?}, sckstopbywr: {=bool:?}, ahbbustimeout: {=bool:?}, seqtimeout: {=bool:?} }}",
            self.ipcmddone(),
            self.ipcmdge(),
            self.ahbcmdge(),
            self.ipcmderr(),
            self.ahbcmderr(),
            self.iprxwa(),
            self.iptxwe(),
            self.datalearnfail(),
            self.sckstopbyrd(),
            self.sckstopbywr(),
            self.ahbbustimeout(),
            self.seqtimeout()
        )
    }
}
#[doc = "IP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "Setting this bit will trigger an IP Command."]
    #[must_use]
    #[inline(always)]
    pub const fn trg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub const fn set_trg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ipcmd {
    #[inline(always)]
    fn default() -> Ipcmd {
        Ipcmd(0)
    }
}
impl core::fmt::Debug for Ipcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcmd").field("trg", &self.trg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipcmd {{ trg: {=bool:?} }}", self.trg())
    }
}
#[doc = "IP Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr0(pub u32);
impl Ipcr0 {
    #[doc = "Serial Flash Address for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn sfar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Serial Flash Address for IP command."]
    #[inline(always)]
    pub const fn set_sfar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipcr0 {
    #[inline(always)]
    fn default() -> Ipcr0 {
        Ipcr0(0)
    }
}
impl core::fmt::Debug for Ipcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr0").field("sfar", &self.sfar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipcr0 {{ sfar: {=u32:?} }}", self.sfar())
    }
}
#[doc = "IP Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Flash Read/Program Data Size (in Bytes) for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn idatsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub const fn set_idatsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn set_iseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub const fn set_iseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Parallel mode Enabled for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn iparen(&self) -> super::vals::Iparen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Iparen::from_bits(val as u8)
    }
    #[doc = "Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub const fn set_iparen(&mut self, val: super::vals::Iparen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ipcr1 {
    #[inline(always)]
    fn default() -> Ipcr1 {
        Ipcr1(0)
    }
}
impl core::fmt::Debug for Ipcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr1")
            .field("idatsz", &self.idatsz())
            .field("iseqid", &self.iseqid())
            .field("iseqnum", &self.iseqnum())
            .field("iparen", &self.iparen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr1 {{ idatsz: {=u16:?}, iseqid: {=u8:?}, iseqnum: {=u8:?}, iparen: {:?} }}",
            self.idatsz(),
            self.iseqid(),
            self.iseqnum(),
            self.iparen()
        )
    }
}
#[doc = "IP RX FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfcr(pub u32);
impl Iprxfcr {
    #[doc = "Clear all valid data entries in IP RX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn clriprxf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub const fn set_clriprxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP RX FIFO reading by DMA enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> super::vals::Rxdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxdmaen::from_bits(val as u8)
    }
    #[doc = "IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub const fn set_rxdmaen(&mut self, val: super::vals::Rxdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watermark level is (RXWMRK+1)*64 Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub const fn set_rxwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
}
impl Default for Iprxfcr {
    #[inline(always)]
    fn default() -> Iprxfcr {
        Iprxfcr(0)
    }
}
impl core::fmt::Debug for Iprxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfcr")
            .field("clriprxf", &self.clriprxf())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxwmrk", &self.rxwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iprxfcr {{ clriprxf: {=bool:?}, rxdmaen: {:?}, rxwmrk: {=u8:?} }}",
            self.clriprxf(),
            self.rxdmaen(),
            self.rxwmrk()
        )
    }
}
#[doc = "IP RX FIFO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfsts(pub u32);
impl Iprxfsts {
    #[doc = "Fill level of IP RX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill level of IP RX FIFO."]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Total Read Data Counter: RDCNTR * 64 Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn rdcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub const fn set_rdcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iprxfsts {
    #[inline(always)]
    fn default() -> Iprxfsts {
        Iprxfsts(0)
    }
}
impl core::fmt::Debug for Iprxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfsts")
            .field("fill", &self.fill())
            .field("rdcntr", &self.rdcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iprxfsts {{ fill: {=u8:?}, rdcntr: {=u16:?} }}",
            self.fill(),
            self.rdcntr()
        )
    }
}
#[doc = "IP TX FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfcr(pub u32);
impl Iptxfcr {
    #[doc = "Clear all valid data entries in IP TX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn clriptxf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub const fn set_clriptxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP TX FIFO filling by DMA enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn txdmaen(&self) -> super::vals::Txdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Txdmaen::from_bits(val as u8)
    }
    #[doc = "IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub const fn set_txdmaen(&mut self, val: super::vals::Txdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watermark level is (TXWMRK+1)*64 Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn txwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub const fn set_txwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for Iptxfcr {
    #[inline(always)]
    fn default() -> Iptxfcr {
        Iptxfcr(0)
    }
}
impl core::fmt::Debug for Iptxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfcr")
            .field("clriptxf", &self.clriptxf())
            .field("txdmaen", &self.txdmaen())
            .field("txwmrk", &self.txwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iptxfcr {{ clriptxf: {=bool:?}, txdmaen: {:?}, txwmrk: {=u8:?} }}",
            self.clriptxf(),
            self.txdmaen(),
            self.txwmrk()
        )
    }
}
#[doc = "IP TX FIFO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfsts(pub u32);
impl Iptxfsts {
    #[doc = "Fill level of IP TX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill level of IP TX FIFO."]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Total Write Data Counter: WRCNTR * 64 Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn wrcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub const fn set_wrcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iptxfsts {
    #[inline(always)]
    fn default() -> Iptxfsts {
        Iptxfsts(0)
    }
}
impl core::fmt::Debug for Iptxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfsts")
            .field("fill", &self.fill())
            .field("wrcntr", &self.wrcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iptxfsts {{ fill: {=u8:?}, wrcntr: {=u16:?} }}",
            self.fill(),
            self.wrcntr()
        )
    }
}
#[doc = "LUT x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut(pub u32);
impl Lut {
    #[doc = "OPERAND0"]
    #[must_use]
    #[inline(always)]
    pub const fn operand0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub const fn set_operand0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NUM_PADS0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS0"]
    #[inline(always)]
    pub const fn set_num_pads0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "OPCODE"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE"]
    #[inline(always)]
    pub const fn set_opcode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "OPERAND1"]
    #[must_use]
    #[inline(always)]
    pub const fn operand1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND1"]
    #[inline(always)]
    pub const fn set_operand1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NUM_PADS1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS1"]
    #[inline(always)]
    pub const fn set_num_pads1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "OPCODE1"]
    #[must_use]
    #[inline(always)]
    pub const fn opcode1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE1"]
    #[inline(always)]
    pub const fn set_opcode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Lut {
    #[inline(always)]
    fn default() -> Lut {
        Lut(0)
    }
}
impl core::fmt::Debug for Lut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lut")
            .field("operand0", &self.operand0())
            .field("num_pads0", &self.num_pads0())
            .field("opcode0", &self.opcode0())
            .field("operand1", &self.operand1())
            .field("num_pads1", &self.num_pads1())
            .field("opcode1", &self.opcode1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lut {{ operand0: {=u8:?}, num_pads0: {=u8:?}, opcode0: {=u8:?}, operand1: {=u8:?}, num_pads1: {=u8:?}, opcode1: {=u8:?} }}",
            self.operand0(),
            self.num_pads0(),
            self.opcode0(),
            self.operand1(),
            self.num_pads1(),
            self.opcode1()
        )
    }
}
#[doc = "LUT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutcr(pub u32);
impl Lutcr {
    #[doc = "Lock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock LUT"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unlock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock LUT"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Lutcr {
    #[inline(always)]
    fn default() -> Lutcr {
        Lutcr(0)
    }
}
impl core::fmt::Debug for Lutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lutcr")
            .field("lock", &self.lock())
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lutcr {{ lock: {=bool:?}, unlock: {=bool:?} }}",
            self.lock(),
            self.unlock()
        )
    }
}
#[doc = "LUT Key Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutkey(pub u32);
impl Lutkey {
    #[doc = "The Key to lock or unlock LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The Key to lock or unlock LUT."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lutkey {
    #[inline(always)]
    fn default() -> Lutkey {
        Lutkey(0)
    }
}
impl core::fmt::Debug for Lutkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lutkey").field("key", &self.key()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutkey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lutkey {{ key: {=u32:?} }}", self.key())
    }
}
#[doc = "Module Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr0(pub u32);
impl Mcr0 {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample Clock source selection for Flash Reading"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc(&self) -> super::vals::Rxclksrc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rxclksrc::from_bits(val as u8)
    }
    #[doc = "Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub const fn set_rxclksrc(&mut self, val: super::vals::Rxclksrc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "The serial root clock could be divided inside FlexSPI wrapper. Refer Clocks chapter for more details on clocking."]
    #[must_use]
    #[inline(always)]
    pub const fn serclkdiv(&self) -> super::vals::Serclkdiv {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Serclkdiv::from_bits(val as u8)
    }
    #[doc = "The serial root clock could be divided inside FlexSPI wrapper. Refer Clocks chapter for more details on clocking."]
    #[inline(always)]
    pub const fn set_serclkdiv(&mut self, val: super::vals::Serclkdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Speed Serial Flash access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsen(&self) -> super::vals::Hsen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hsen::from_bits(val as u8)
    }
    #[doc = "Half Speed Serial Flash access Enable."]
    #[inline(always)]
    pub const fn set_hsen(&mut self, val: super::vals::Hsen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze mode enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze mode enable bit"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is used to force SCLK output free-running. For FPGA applications, external device may use SCLK as reference clock to its internal PLL. If SCLK free-running is enabled, data sampling with loopback clock from SCLK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[must_use]
    #[inline(always)]
    pub const fn sckfreerunen(&self) -> super::vals::Sckfreerunen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sckfreerunen::from_bits(val as u8)
    }
    #[doc = "This bit is used to force SCLK output free-running. For FPGA applications, external device may use SCLK as reference clock to its internal PLL. If SCLK free-running is enabled, data sampling with loopback clock from SCLK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[inline(always)]
    pub const fn set_sckfreerunen(&mut self, val: super::vals::Sckfreerunen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is used to enable/disable data learning feature. When data learning is disabled, the sampling clock phase 0 is always used for RX data sampling even if LEARN instruction is correctly executed."]
    #[must_use]
    #[inline(always)]
    pub const fn learnen(&self) -> super::vals::Learnen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Learnen::from_bits(val as u8)
    }
    #[doc = "This bit is used to enable/disable data learning feature. When data learning is disabled, the sampling clock phase 0 is always used for RX data sampling even if LEARN instruction is correctly executed."]
    #[inline(always)]
    pub const fn set_learnen(&mut self, val: super::vals::Learnen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Time out wait cycle for IP command grant."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgrantwait(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Time out wait cycle for IP command grant."]
    #[inline(always)]
    pub const fn set_ipgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Timeout wait cycle for AHB command grant."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgrantwait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub const fn set_ahbgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr0 {
    #[inline(always)]
    fn default() -> Mcr0 {
        Mcr0(0)
    }
}
impl core::fmt::Debug for Mcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr0")
            .field("swreset", &self.swreset())
            .field("mdis", &self.mdis())
            .field("rxclksrc", &self.rxclksrc())
            .field("serclkdiv", &self.serclkdiv())
            .field("hsen", &self.hsen())
            .field("dozeen", &self.dozeen())
            .field("sckfreerunen", &self.sckfreerunen())
            .field("learnen", &self.learnen())
            .field("ipgrantwait", &self.ipgrantwait())
            .field("ahbgrantwait", &self.ahbgrantwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr0 {{ swreset: {=bool:?}, mdis: {=bool:?}, rxclksrc: {:?}, serclkdiv: {:?}, hsen: {:?}, dozeen: {:?}, sckfreerunen: {:?}, learnen: {:?}, ipgrantwait: {=u8:?}, ahbgrantwait: {=u8:?} }}",
            self.swreset(),
            self.mdis(),
            self.rxclksrc(),
            self.serclkdiv(),
            self.hsen(),
            self.dozeen(),
            self.sckfreerunen(),
            self.learnen(),
            self.ipgrantwait(),
            self.ahbgrantwait()
        )
    }
}
#[doc = "Module Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr1(pub u32);
impl Mcr1 {
    #[doc = "AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuswait(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub const fn set_ahbbuswait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn seqwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub const fn set_seqwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mcr1 {
    #[inline(always)]
    fn default() -> Mcr1 {
        Mcr1(0)
    }
}
impl core::fmt::Debug for Mcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr1")
            .field("ahbbuswait", &self.ahbbuswait())
            .field("seqwait", &self.seqwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr1 {{ ahbbuswait: {=u16:?}, seqwait: {=u16:?} }}",
            self.ahbbuswait(),
            self.seqwait()
        )
    }
}
#[doc = "Module Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr2(pub u32);
impl Mcr2 {
    #[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbbufopt(&self) -> super::vals::Clrahbbufopt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrahbbufopt::from_bits(val as u8)
    }
    #[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub const fn set_clrahbbufopt(&mut self, val: super::vals::Clrahbbufopt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn clrlearnphase(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub const fn set_clrlearnphase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[must_use]
    #[inline(always)]
    pub const fn samedeviceen(&self) -> super::vals::Samedeviceen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Samedeviceen::from_bits(val as u8)
    }
    #[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub const fn set_samedeviceen(&mut self, val: super::vals::Samedeviceen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\] should be set."]
    #[must_use]
    #[inline(always)]
    pub const fn sckbdiffopt(&self) -> super::vals::Sckbdiffopt {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sckbdiffopt::from_bits(val as u8)
    }
    #[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\] should be set."]
    #[inline(always)]
    pub const fn set_sckbdiffopt(&mut self, val: super::vals::Sckbdiffopt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[must_use]
    #[inline(always)]
    pub const fn resumewait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub const fn set_resumewait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr2 {
    #[inline(always)]
    fn default() -> Mcr2 {
        Mcr2(0)
    }
}
impl core::fmt::Debug for Mcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr2")
            .field("clrahbbufopt", &self.clrahbbufopt())
            .field("clrlearnphase", &self.clrlearnphase())
            .field("samedeviceen", &self.samedeviceen())
            .field("sckbdiffopt", &self.sckbdiffopt())
            .field("resumewait", &self.resumewait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr2 {{ clrahbbufopt: {:?}, clrlearnphase: {=bool:?}, samedeviceen: {:?}, sckbdiffopt: {:?}, resumewait: {=u8:?} }}",
            self.clrahbbufopt(),
            self.clrlearnphase(),
            self.samedeviceen(),
            self.sckbdiffopt(),
            self.resumewait()
        )
    }
}
#[doc = "IP RX FIFO Data Register x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfdr(pub u32);
impl Rfdr {
    #[doc = "RX Data"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Data"]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rfdr {
    #[inline(always)]
    fn default() -> Rfdr {
        Rfdr(0)
    }
}
impl core::fmt::Debug for Rfdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfdr")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rfdr {{ rxdata: {=u32:?} }}", self.rxdata())
    }
}
#[doc = "Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn seqidle(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub const fn set_seqidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[must_use]
    #[inline(always)]
    pub const fn arbidle(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub const fn set_arbidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[must_use]
    #[inline(always)]
    pub const fn arbcmdsrc(&self) -> super::vals::Arbcmdsrc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Arbcmdsrc::from_bits(val as u8)
    }
    #[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub const fn set_arbcmdsrc(&mut self, val: super::vals::Arbcmdsrc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphasea(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[inline(always)]
    pub const fn set_datalearnphasea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicate the sampling clock phase selection on Port B after Data Learning."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphaseb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the sampling clock phase selection on Port B after Data Learning."]
    #[inline(always)]
    pub const fn set_datalearnphaseb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(0)
    }
}
impl core::fmt::Debug for Sts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts0")
            .field("seqidle", &self.seqidle())
            .field("arbidle", &self.arbidle())
            .field("arbcmdsrc", &self.arbcmdsrc())
            .field("datalearnphasea", &self.datalearnphasea())
            .field("datalearnphaseb", &self.datalearnphaseb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts0 {{ seqidle: {=bool:?}, arbidle: {=bool:?}, arbcmdsrc: {:?}, datalearnphasea: {=u8:?}, datalearnphaseb: {=u8:?} }}",
            self.seqidle(),
            self.arbidle(),
            self.arbcmdsrc(),
            self.datalearnphasea(),
            self.datalearnphaseb()
        )
    }
}
#[doc = "Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts1(pub u32);
impl Sts1 {
    #[doc = "Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn set_ahbcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrcode(&self) -> super::vals::Ahbcmderrcode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ahbcmderrcode::from_bits(val as u8)
    }
    #[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn set_ahbcmderrcode(&mut self, val: super::vals::Ahbcmderrcode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn set_ipcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrcode(&self) -> super::vals::Ipcmderrcode {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ipcmderrcode::from_bits(val as u8)
    }
    #[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn set_ipcmderrcode(&mut self, val: super::vals::Ipcmderrcode) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sts1 {
    #[inline(always)]
    fn default() -> Sts1 {
        Sts1(0)
    }
}
impl core::fmt::Debug for Sts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts1")
            .field("ahbcmderrid", &self.ahbcmderrid())
            .field("ahbcmderrcode", &self.ahbcmderrcode())
            .field("ipcmderrid", &self.ipcmderrid())
            .field("ipcmderrcode", &self.ipcmderrcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts1 {{ ahbcmderrid: {=u8:?}, ahbcmderrcode: {:?}, ipcmderrid: {=u8:?}, ipcmderrcode: {:?} }}",
            self.ahbcmderrid(),
            self.ahbcmderrcode(),
            self.ipcmderrid(),
            self.ipcmderrcode()
        )
    }
}
#[doc = "Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "Flash A sample clock slave delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn aslvlock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn set_aslvlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash A sample clock reference delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn areflock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn set_areflock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash A sample clock slave delay line delay cell number selection ."]
    #[must_use]
    #[inline(always)]
    pub const fn aslvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub const fn set_aslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Flash A sample clock reference delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn arefsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_arefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Flash B sample clock slave delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn bslvlock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn set_bslvlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flash B sample clock reference delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn breflock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn set_breflock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flash B sample clock slave delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn bslvsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_bslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Flash B sample clock reference delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn brefsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_brefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(0)
    }
}
impl core::fmt::Debug for Sts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts2")
            .field("aslvlock", &self.aslvlock())
            .field("areflock", &self.areflock())
            .field("aslvsel", &self.aslvsel())
            .field("arefsel", &self.arefsel())
            .field("bslvlock", &self.bslvlock())
            .field("breflock", &self.breflock())
            .field("bslvsel", &self.bslvsel())
            .field("brefsel", &self.brefsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts2 {{ aslvlock: {=bool:?}, areflock: {=bool:?}, aslvsel: {=u8:?}, arefsel: {=u8:?}, bslvlock: {=bool:?}, breflock: {=bool:?}, bslvsel: {=u8:?}, brefsel: {=u8:?} }}",
            self.aslvlock(),
            self.areflock(),
            self.aslvsel(),
            self.arefsel(),
            self.bslvlock(),
            self.breflock(),
            self.bslvsel(),
            self.brefsel()
        )
    }
}
#[doc = "IP TX FIFO Data Register x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfdr(pub u32);
impl Tfdr {
    #[doc = "TX Data"]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TX Data"]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tfdr {
    #[inline(always)]
    fn default() -> Tfdr {
        Tfdr(0)
    }
}
impl core::fmt::Debug for Tfdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfdr")
            .field("txdata", &self.txdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tfdr {{ txdata: {=u32:?} }}", self.txdata())
    }
}
