#[doc = "Receive Buffer Region 0 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend0(pub u32);
impl Ahbbufregionend0 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend0 {
    #[inline(always)]
    fn default() -> Ahbbufregionend0 {
        Ahbbufregionend0(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend0")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend0 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 1 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend1(pub u32);
impl Ahbbufregionend1 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend1 {
    #[inline(always)]
    fn default() -> Ahbbufregionend1 {
        Ahbbufregionend1(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend1")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend1 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 2 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend2(pub u32);
impl Ahbbufregionend2 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend2 {
    #[inline(always)]
    fn default() -> Ahbbufregionend2 {
        Ahbbufregionend2(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend2")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend2 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 3 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend3(pub u32);
impl Ahbbufregionend3 {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend3 {
    #[inline(always)]
    fn default() -> Ahbbufregionend3 {
        Ahbbufregionend3(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend3")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend3 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart0(pub u32);
impl Ahbbufregionstart0 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart0 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart0 {
        Ahbbufregionstart0(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart0")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart0 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart1(pub u32);
impl Ahbbufregionstart1 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart1 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart1 {
        Ahbbufregionstart1(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart1")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart1 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart2(pub u32);
impl Ahbbufregionstart2 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart2 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart2 {
        Ahbbufregionstart2(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart2")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart2 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart3(pub u32);
impl Ahbbufregionstart3 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart3 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart3 {
        Ahbbufregionstart3(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart3")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart3 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "AHB Bus Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbcr(pub u32);
impl Ahbcr {
    #[doc = "AHB Parallel Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aparen(&self) -> super::vals::Aparen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aparen::from_bits(val as u8)
    }
    #[doc = "AHB Parallel Mode Enable"]
    #[inline(always)]
    pub const fn set_aparen(&mut self, val: super::vals::Aparen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear AHB Receive Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbrxbuf(&self) -> super::vals::Clrahbrxbuf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Clrahbrxbuf::from_bits(val as u8)
    }
    #[doc = "Clear AHB Receive Buffer"]
    #[inline(always)]
    pub const fn set_clrahbrxbuf(&mut self, val: super::vals::Clrahbrxbuf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear AHB Transmit Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbtxbuf(&self) -> super::vals::Clrahbtxbuf {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Clrahbtxbuf::from_bits(val as u8)
    }
    #[doc = "Clear AHB Transmit Buffer"]
    #[inline(always)]
    pub const fn set_clrahbtxbuf(&mut self, val: super::vals::Clrahbtxbuf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Cacheable Read Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cachableen(&self) -> super::vals::Cachableen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cachableen::from_bits(val as u8)
    }
    #[doc = "Cacheable Read Access Enable"]
    #[inline(always)]
    pub const fn set_cachableen(&mut self, val: super::vals::Cachableen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bufferable Write Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bufferableen(&self) -> super::vals::Bufferableen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bufferableen::from_bits(val as u8)
    }
    #[doc = "Bufferable Write Access Enable"]
    #[inline(always)]
    pub const fn set_bufferableen(&mut self, val: super::vals::Bufferableen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::AhbcrPrefetchen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AhbcrPrefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::AhbcrPrefetchen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "AHB Read Address Option"]
    #[must_use]
    #[inline(always)]
    pub const fn readaddropt(&self) -> super::vals::Readaddropt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Readaddropt::from_bits(val as u8)
    }
    #[doc = "AHB Read Address Option"]
    #[inline(always)]
    pub const fn set_readaddropt(&mut self, val: super::vals::Readaddropt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Read Resume Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn resumedisable(&self) -> super::vals::Resumedisable {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Resumedisable::from_bits(val as u8)
    }
    #[doc = "AHB Read Resume Disable"]
    #[inline(always)]
    pub const fn set_resumedisable(&mut self, val: super::vals::Resumedisable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AHB Read Size Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn readszalign(&self) -> super::vals::Readszalign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Readszalign::from_bits(val as u8)
    }
    #[doc = "AHB Read Size Alignment"]
    #[inline(always)]
    pub const fn set_readszalign(&mut self, val: super::vals::Readszalign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AHB Boundary Alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn alignment(&self) -> super::vals::Alignment {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Alignment::from_bits(val as u8)
    }
    #[doc = "AHB Boundary Alignment"]
    #[inline(always)]
    pub const fn set_alignment(&mut self, val: super::vals::Alignment) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "AHB Memory-Mapped Flash Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn aflashbase(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Memory-Mapped Flash Base Address"]
    #[inline(always)]
    pub const fn set_aflashbase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
            .field("clrahbrxbuf", &self.clrahbrxbuf())
            .field("clrahbtxbuf", &self.clrahbtxbuf())
            .field("cachableen", &self.cachableen())
            .field("bufferableen", &self.bufferableen())
            .field("prefetchen", &self.prefetchen())
            .field("readaddropt", &self.readaddropt())
            .field("resumedisable", &self.resumedisable())
            .field("readszalign", &self.readszalign())
            .field("alignment", &self.alignment())
            .field("aflashbase", &self.aflashbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbcr {{ aparen: {:?}, clrahbrxbuf: {:?}, clrahbtxbuf: {:?}, cachableen: {:?}, bufferableen: {:?}, prefetchen: {:?}, readaddropt: {:?}, resumedisable: {:?}, readszalign: {:?}, alignment: {:?}, aflashbase: {=u8:?} }}",
            self.aparen(),
            self.clrahbrxbuf(),
            self.clrahbtxbuf(),
            self.cachableen(),
            self.bufferableen(),
            self.prefetchen(),
            self.readaddropt(),
            self.resumedisable(),
            self.readszalign(),
            self.alignment(),
            self.aflashbase()
        )
    }
}
#[doc = "AHB Receive Buffer 0 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf0cr0(pub u32);
impl Ahbrxbuf0cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf0cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf0cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf0cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf0cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf0cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf0cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf0cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf0cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 1 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf1cr0(pub u32);
impl Ahbrxbuf1cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf1cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf1cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf1cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf1cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf1cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf1cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf1cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 2 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf2cr0(pub u32);
impl Ahbrxbuf2cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf2cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf2cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf2cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf2cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf2cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf2cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf2cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 3 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf3cr0(pub u32);
impl Ahbrxbuf3cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf3cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf3cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf3cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf3cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf3cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf3cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf3cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf3cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 4 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf4cr0(pub u32);
impl Ahbrxbuf4cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf4cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf4cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf4cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf4cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf4cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf4cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf4cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf4cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 5 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf5cr0(pub u32);
impl Ahbrxbuf5cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf5cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf5cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf5cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf5cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf5cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf5cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf5cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf5cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 6 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf6cr0(pub u32);
impl Ahbrxbuf6cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf6cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf6cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf6cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf6cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf6cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf6cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf6cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf6cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 7 Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf7cr0(pub u32);
impl Ahbrxbuf7cr0 {
    #[doc = "AHB Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID"]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority"]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> super::vals::Ahbrxbuf7cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ahbrxbuf7cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: super::vals::Ahbrxbuf7cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> super::vals::Ahbrxbuf7cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ahbrxbuf7cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable"]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: super::vals::Ahbrxbuf7cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf7cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf7cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Suspend Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbspndsts(pub u32);
impl Ahbspndsts {
    #[doc = "Active AHB Read Prefetch Suspended"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "Active AHB Read Prefetch Suspended"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn bufid(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    #[inline(always)]
    pub const fn set_bufid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Data Left"]
    #[must_use]
    #[inline(always)]
    pub const fn datlft(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Left"]
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
            "Ahbspndsts {{ active: {:?}, bufid: {=u8:?}, datlft: {=u16:?} }}",
            self.active(),
            self.bufid(),
            self.datlft()
        )
    }
}
#[doc = "DLL Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL Calibration Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> super::vals::Dllen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dllen::from_bits(val as u8)
    }
    #[doc = "DLL Calibration Enable"]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: super::vals::Dllen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> super::vals::Dllreset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dllreset::from_bits(val as u8)
    }
    #[doc = "DLL reset"]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: super::vals::Dllreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Target Delay Line"]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Target Delay Line"]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> super::vals::Ovrden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ovrden::from_bits(val as u8)
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: super::vals::Ovrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Target Clock Delay Line Override Value"]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[must_use]
    #[inline(always)]
    pub const fn refphasegap(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[inline(always)]
    pub const fn set_refphasegap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
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
            .field("refphasegap", &self.refphasegap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dllcr {{ dllen: {:?}, dllreset: {:?}, slvdlytarget: {=u8:?}, ovrden: {:?}, ovrdval: {=u8:?}, refphasegap: {=u8:?} }}",
            self.dllen(),
            self.dllreset(),
            self.slvdlytarget(),
            self.ovrden(),
            self.ovrdval(),
            self.refphasegap()
        )
    }
}
#[doc = "Data Learning Pattern"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlpr(pub u32);
impl Dlpr {
    #[doc = "Data Learning Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn dlp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Learning Pattern"]
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
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha1cr0(pub u32);
impl Flsha1cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("addrshift", &self.addrshift())
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
        )
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha2cr0(pub u32);
impl Flsha2cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("addrshift", &self.addrshift())
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
        )
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb1cr0(pub u32);
impl Flshb1cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("addrshift", &self.addrshift())
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
        )
    }
}
#[doc = "Flash Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb2cr0(pub u32);
impl Flshb2cr0 {
    #[doc = "Flash Size in KB"]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB"]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control"]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control"]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable"]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("addrshift", &self.addrshift())
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
        )
    }
}
#[doc = "Flash Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr1(pub u32);
impl Flshcr1 {
    #[doc = "Serial Flash CS Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn tcss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Setup Time"]
    #[inline(always)]
    pub const fn set_tcss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn tcsh(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Hold Time"]
    #[inline(always)]
    pub const fn set_tcsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Word-Addressable"]
    #[must_use]
    #[inline(always)]
    pub const fn wa(&self) -> super::vals::Wa {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wa::from_bits(val as u8)
    }
    #[doc = "Word-Addressable"]
    #[inline(always)]
    pub const fn set_wa(&mut self, val: super::vals::Wa) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size"]
    #[must_use]
    #[inline(always)]
    pub const fn cas(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size"]
    #[inline(always)]
    pub const fn set_cas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Chip Select Interval Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn csintervalunit(&self) -> super::vals::Csintervalunit {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Csintervalunit::from_bits(val as u8)
    }
    #[doc = "Chip Select Interval Unit"]
    #[inline(always)]
    pub const fn set_csintervalunit(&mut self, val: super::vals::Csintervalunit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Chip Select Interval"]
    #[must_use]
    #[inline(always)]
    pub const fn csinterval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip Select Interval"]
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
            "Flshcr1 {{ tcss: {=u8:?}, tcsh: {=u8:?}, wa: {:?}, cas: {=u8:?}, csintervalunit: {:?}, csinterval: {=u16:?} }}",
            self.tcss(),
            self.tcsh(),
            self.wa(),
            self.cas(),
            self.csintervalunit(),
            self.csinterval()
        )
    }
}
#[doc = "Flash Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr2(pub u32);
impl Flshcr2 {
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    #[inline(always)]
    pub const fn set_ardseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqnum(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    #[inline(always)]
    pub const fn set_ardseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    #[inline(always)]
    pub const fn set_awrseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    #[inline(always)]
    pub const fn set_awrseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "AHB Write Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "AHB Write Wait"]
    #[inline(always)]
    pub const fn set_awrwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AWRWAIT Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn awrwaitunit(&self) -> super::vals::Awrwaitunit {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Awrwaitunit::from_bits(val as u8)
    }
    #[doc = "AWRWAIT Unit"]
    #[inline(always)]
    pub const fn set_awrwaitunit(&mut self, val: super::vals::Awrwaitunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Clear Instruction Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrinstrptr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Instruction Pointer"]
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
#[doc = "Flash Control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr4(pub u32);
impl Flshcr4 {
    #[doc = "Write Mask Option 1"]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Mask Option 1"]
    #[inline(always)]
    pub const fn set_wmopt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write Mask Enable for Port A"]
    #[must_use]
    #[inline(always)]
    pub const fn wmena(&self) -> super::vals::Wmena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wmena::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port A"]
    #[inline(always)]
    pub const fn set_wmena(&mut self, val: super::vals::Wmena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Mask Enable for Port B"]
    #[must_use]
    #[inline(always)]
    pub const fn wmenb(&self) -> super::vals::Wmenb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wmenb::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port B"]
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
            "Flshcr4 {{ wmopt1: {=bool:?}, wmena: {:?}, wmenb: {:?} }}",
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
    #[doc = "End Address of HADDR Remap Range"]
    #[must_use]
    #[inline(always)]
    pub const fn endstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of HADDR Remap Range"]
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
#[doc = "HADDR Remap Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddroffset(pub u32);
impl Haddroffset {
    #[doc = "HADDR Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn addroffset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR Offset"]
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
#[doc = "HADDR REMAP Start Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddrstart(pub u32);
impl Haddrstart {
    #[doc = "AHB Bus Address Remap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn remapen(&self) -> super::vals::Remapen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Remapen::from_bits(val as u8)
    }
    #[doc = "AHB Bus Address Remap Enable"]
    #[inline(always)]
    pub const fn set_remapen(&mut self, val: super::vals::Remapen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HADDR Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addrstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR Start Address"]
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
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> super::vals::Ipcmddoneen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipcmddoneen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: super::vals::Ipcmddoneen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdgeen(&self) -> super::vals::Ipcmdgeen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipcmdgeen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmdgeen(&mut self, val: super::vals::Ipcmdgeen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdgeen(&self) -> super::vals::Ahbcmdgeen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ahbcmdgeen::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbcmdgeen(&mut self, val: super::vals::Ahbcmdgeen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> super::vals::Ipcmderren {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ipcmderren::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: super::vals::Ipcmderren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderren(&self) -> super::vals::Ahbcmderren {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ahbcmderren::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ahbcmderren(&mut self, val: super::vals::Ahbcmderren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwaen(&self) -> super::vals::Iprxwaen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Iprxwaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iprxwaen(&mut self, val: super::vals::Iprxwaen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iptxween(&self) -> super::vals::Iptxween {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Iptxween::from_bits(val as u8)
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iptxween(&mut self, val: super::vals::Iptxween) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning Failed Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfailen(&self) -> super::vals::Datalearnfailen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Datalearnfailen::from_bits(val as u8)
    }
    #[doc = "Data Learning Failed Interrupt Enable"]
    #[inline(always)]
    pub const fn set_datalearnfailen(&mut self, val: super::vals::Datalearnfailen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrden(&self) -> super::vals::Sckstopbyrden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sckstopbyrden::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sckstopbyrden(&mut self, val: super::vals::Sckstopbyrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywren(&self) -> super::vals::Sckstopbywren {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Sckstopbywren::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sckstopbywren(&mut self, val: super::vals::Sckstopbywren) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeouten(&self) -> super::vals::Ahbbustimeouten {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ahbbustimeouten::from_bits(val as u8)
    }
    #[doc = "AHB Bus Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ahbbustimeouten(&mut self, val: super::vals::Ahbbustimeouten) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeouten(&self) -> super::vals::Seqtimeouten {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Seqtimeouten::from_bits(val as u8)
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seqtimeouten(&mut self, val: super::vals::Seqtimeouten) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "IP Command Security Violation Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdsecurevioen(&self) -> super::vals::Ipcmdsecurevioen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ipcmdsecurevioen::from_bits(val as u8)
    }
    #[doc = "IP Command Security Violation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ipcmdsecurevioen(&mut self, val: super::vals::Ipcmdsecurevioen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "AHB Read GCM Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmerren(&self) -> super::vals::Ahbgcmerren {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ahbgcmerren::from_bits(val as u8)
    }
    #[doc = "AHB Read GCM Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ahbgcmerren(&mut self, val: super::vals::Ahbgcmerren) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
            .field("ipcmdsecurevioen", &self.ipcmdsecurevioen())
            .field("ahbgcmerren", &self.ahbgcmerren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ipcmddoneen: {:?}, ipcmdgeen: {:?}, ahbcmdgeen: {:?}, ipcmderren: {:?}, ahbcmderren: {:?}, iprxwaen: {:?}, iptxween: {:?}, datalearnfailen: {:?}, sckstopbyrden: {:?}, sckstopbywren: {:?}, ahbbustimeouten: {:?}, seqtimeouten: {:?}, ipcmdsecurevioen: {:?}, ahbgcmerren: {:?} }}",
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
            self.seqtimeouten(),
            self.ipcmdsecurevioen(),
            self.ahbgcmerren()
        )
    }
}
#[doc = "Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdge(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    #[inline(always)]
    pub const fn set_ipcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    #[inline(always)]
    pub const fn set_ahbcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    #[inline(always)]
    pub const fn set_ahbcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    #[inline(always)]
    pub const fn set_iprxwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn iptxwe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    #[inline(always)]
    pub const fn set_iptxwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning Failed"]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning Failed"]
    #[inline(always)]
    pub const fn set_datalearnfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    #[inline(always)]
    pub const fn set_sckstopbyrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    #[inline(always)]
    pub const fn set_sckstopbywr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeout(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus Timeout"]
    #[inline(always)]
    pub const fn set_ahbbustimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence Execution Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeout(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Execution Timeout"]
    #[inline(always)]
    pub const fn set_seqtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IP Command Security Violation"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdsecurevio(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IP Command Security Violation"]
    #[inline(always)]
    pub const fn set_ipcmdsecurevio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "AHB Read GCM Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmerr(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read GCM Error"]
    #[inline(always)]
    pub const fn set_ahbgcmerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("ipcmdsecurevio", &self.ipcmdsecurevio())
            .field("ahbgcmerr", &self.ahbgcmerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intr {{ ipcmddone: {=bool:?}, ipcmdge: {=bool:?}, ahbcmdge: {=bool:?}, ipcmderr: {=bool:?}, ahbcmderr: {=bool:?}, iprxwa: {=bool:?}, iptxwe: {=bool:?}, datalearnfail: {=bool:?}, sckstopbyrd: {=bool:?}, sckstopbywr: {=bool:?}, ahbbustimeout: {=bool:?}, seqtimeout: {=bool:?}, ipcmdsecurevio: {=bool:?}, ahbgcmerr: {=bool:?} }}",
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
            self.seqtimeout(),
            self.ipcmdsecurevio(),
            self.ahbgcmerr()
        )
    }
}
#[doc = "IP Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "Command Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trg(&self) -> super::vals::Trg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trg::from_bits(val as u8)
    }
    #[doc = "Command Trigger"]
    #[inline(always)]
    pub const fn set_trg(&mut self, val: super::vals::Trg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        defmt::write!(f, "Ipcmd {{ trg: {:?} }}", self.trg())
    }
}
#[doc = "IP Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr0(pub u32);
impl Ipcr0 {
    #[doc = "Serial Flash Address"]
    #[must_use]
    #[inline(always)]
    pub const fn sfar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Serial Flash Address"]
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
#[doc = "IP Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn idatsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[inline(always)]
    pub const fn set_idatsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn set_iseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
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
    #[doc = "Parallel Mode Enable for IP Commands"]
    #[must_use]
    #[inline(always)]
    pub const fn iparen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Mode Enable for IP Commands"]
    #[inline(always)]
    pub const fn set_iparen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            "Ipcr1 {{ idatsz: {=u16:?}, iseqid: {=u8:?}, iseqnum: {=u8:?}, iparen: {=bool:?} }}",
            self.idatsz(),
            self.iseqid(),
            self.iseqnum(),
            self.iparen()
        )
    }
}
#[doc = "IP Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr2(pub u32);
impl Ipcr2 {
    #[doc = "IP Command Blocking AHB Command Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkahbreq(&self) -> super::vals::Ipblkahbreq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipblkahbreq::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking AHB Command Request Enable"]
    #[inline(always)]
    pub const fn set_ipblkahbreq(&mut self, val: super::vals::Ipblkahbreq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP Command Blocking AHB Command Acknowledgment Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkahback(&self) -> super::vals::Ipblkahback {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipblkahback::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking AHB Command Acknowledgment Enable"]
    #[inline(always)]
    pub const fn set_ipblkahback(&mut self, val: super::vals::Ipblkahback) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Command Blocking All AHB Command Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkallahb(&self) -> super::vals::Ipblkallahb {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ipblkallahb::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking All AHB Command Enable"]
    #[inline(always)]
    pub const fn set_ipblkallahb(&mut self, val: super::vals::Ipblkallahb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ipcr2 {
    #[inline(always)]
    fn default() -> Ipcr2 {
        Ipcr2(0)
    }
}
impl core::fmt::Debug for Ipcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr2")
            .field("ipblkahbreq", &self.ipblkahbreq())
            .field("ipblkahback", &self.ipblkahback())
            .field("ipblkallahb", &self.ipblkallahb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr2 {{ ipblkahbreq: {:?}, ipblkahback: {:?}, ipblkallahb: {:?} }}",
            self.ipblkahbreq(),
            self.ipblkahback(),
            self.ipblkallahb()
        )
    }
}
#[doc = "IPED Function Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctrl(pub u32);
impl Ipedctrl {
    #[doc = "IPED Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn config(&self) -> super::vals::Config {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Config::from_bits(val as u8)
    }
    #[doc = "IPED Mode Select"]
    #[inline(always)]
    pub const fn set_config(&mut self, val: super::vals::Config) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IPED Encryption and Decryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iped_en(&self) -> super::vals::IpedEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IpedEn::from_bits(val as u8)
    }
    #[doc = "IPED Encryption and Decryption Enable"]
    #[inline(always)]
    pub const fn set_iped_en(&mut self, val: super::vals::IpedEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Write IPED CTR Mode Encryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipwr_en(&self) -> super::vals::IpwrEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IpwrEn::from_bits(val as u8)
    }
    #[doc = "IP Write IPED CTR Mode Encryption Enable"]
    #[inline(always)]
    pub const fn set_ipwr_en(&mut self, val: super::vals::IpwrEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AHB Write IPED CTR Mode Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbwr_en(&self) -> super::vals::AhbwrEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::AhbwrEn::from_bits(val as u8)
    }
    #[doc = "AHB Write IPED CTR Mode Encryption Enable."]
    #[inline(always)]
    pub const fn set_ahbwr_en(&mut self, val: super::vals::AhbwrEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB Read IPED CTR Mode Decryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbrd_en(&self) -> super::vals::AhbrdEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AhbrdEn::from_bits(val as u8)
    }
    #[doc = "AHB Read IPED CTR Mode Decryption Enable"]
    #[inline(always)]
    pub const fn set_ahbrd_en(&mut self, val: super::vals::AhbrdEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Write GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgcmwr(&self) -> super::vals::Ipgcmwr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ipgcmwr::from_bits(val as u8)
    }
    #[doc = "IP Write GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_ipgcmwr(&mut self, val: super::vals::Ipgcmwr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Write IPED GCM Mode Encryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahgcmwr(&self) -> super::vals::Ahgcmwr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ahgcmwr::from_bits(val as u8)
    }
    #[doc = "AHB Write IPED GCM Mode Encryption Enable"]
    #[inline(always)]
    pub const fn set_ahgcmwr(&mut self, val: super::vals::Ahgcmwr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AHB Read IPED GCM Mode Decryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmrd(&self) -> super::vals::Ahbgcmrd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ahbgcmrd::from_bits(val as u8)
    }
    #[doc = "AHB Read IPED GCM Mode Decryption Enable"]
    #[inline(always)]
    pub const fn set_ahbgcmrd(&mut self, val: super::vals::Ahbgcmrd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "IPED Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn iped_protect(&self) -> super::vals::IpedProtect {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::IpedProtect::from_bits(val as u8)
    }
    #[doc = "IPED Protection"]
    #[inline(always)]
    pub const fn set_iped_protect(&mut self, val: super::vals::IpedProtect) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Abort Current Decryption or Encryption"]
    #[must_use]
    #[inline(always)]
    pub const fn iped_swreset(&self) -> super::vals::IpedSwreset {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::IpedSwreset::from_bits(val as u8)
    }
    #[doc = "Abort Current Decryption or Encryption"]
    #[inline(always)]
    pub const fn set_iped_swreset(&mut self, val: super::vals::IpedSwreset) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Ipedctrl {
    #[inline(always)]
    fn default() -> Ipedctrl {
        Ipedctrl(0)
    }
}
impl core::fmt::Debug for Ipedctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctrl")
            .field("config", &self.config())
            .field("iped_en", &self.iped_en())
            .field("ipwr_en", &self.ipwr_en())
            .field("ahbwr_en", &self.ahbwr_en())
            .field("ahbrd_en", &self.ahbrd_en())
            .field("ipgcmwr", &self.ipgcmwr())
            .field("ahgcmwr", &self.ahgcmwr())
            .field("ahbgcmrd", &self.ahbgcmrd())
            .field("iped_protect", &self.iped_protect())
            .field("iped_swreset", &self.iped_swreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctrl {{ config: {:?}, iped_en: {:?}, ipwr_en: {:?}, ahbwr_en: {:?}, ahbrd_en: {:?}, ipgcmwr: {:?}, ahgcmwr: {:?}, ahbgcmrd: {:?}, iped_protect: {:?}, iped_swreset: {:?} }}",
            self.config(),
            self.iped_en(),
            self.ipwr_en(),
            self.ahbwr_en(),
            self.ahbrd_en(),
            self.ipgcmwr(),
            self.ahgcmwr(),
            self.ahbgcmrd(),
            self.iped_protect(),
            self.iped_swreset()
        )
    }
}
#[doc = "IPED Context0 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0aad0(pub u32);
impl Ipedctx0aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx0_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0aad0 {
    #[inline(always)]
    fn default() -> Ipedctx0aad0 {
        Ipedctx0aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx0aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0aad0")
            .field("ctx0_aad0", &self.ctx0_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0aad0 {{ ctx0_aad0: {=u32:?} }}",
            self.ctx0_aad0()
        )
    }
}
#[doc = "IPED Context0 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0aad1(pub u32);
impl Ipedctx0aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx0_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0aad1 {
    #[inline(always)]
    fn default() -> Ipedctx0aad1 {
        Ipedctx0aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx0aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0aad1")
            .field("ctx0_aad1", &self.ctx0_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0aad1 {{ ctx0_aad1: {=u32:?} }}",
            self.ctx0_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0end(pub u32);
impl Ipedctx0end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx0end {
    #[inline(always)]
    fn default() -> Ipedctx0end {
        Ipedctx0end(0)
    }
}
impl core::fmt::Debug for Ipedctx0end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context0 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0iv0(pub u32);
impl Ipedctx0iv0 {
    #[doc = "Lowest 32 bits of IV for region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 0."]
    #[inline(always)]
    pub const fn set_ctx0_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0iv0 {
    #[inline(always)]
    fn default() -> Ipedctx0iv0 {
        Ipedctx0iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx0iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0iv0")
            .field("ctx0_iv0", &self.ctx0_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx0iv0 {{ ctx0_iv0: {=u32:?} }}", self.ctx0_iv0())
    }
}
#[doc = "IPED Context0 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0iv1(pub u32);
impl Ipedctx0iv1 {
    #[doc = "Highest 32 bits of IV for region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 0."]
    #[inline(always)]
    pub const fn set_ctx0_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0iv1 {
    #[inline(always)]
    fn default() -> Ipedctx0iv1 {
        Ipedctx0iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx0iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0iv1")
            .field("ctx0_iv1", &self.ctx0_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx0iv1 {{ ctx0_iv1: {=u32:?} }}", self.ctx0_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0start(pub u32);
impl Ipedctx0start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx0startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx0startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx0startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx0startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx0startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx0startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx0start {
    #[inline(always)]
    fn default() -> Ipedctx0start {
        Ipedctx0start(0)
    }
}
impl core::fmt::Debug for Ipedctx0start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context1 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1aad0(pub u32);
impl Ipedctx1aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx1_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1aad0 {
    #[inline(always)]
    fn default() -> Ipedctx1aad0 {
        Ipedctx1aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx1aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1aad0")
            .field("ctx1_aad0", &self.ctx1_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1aad0 {{ ctx1_aad0: {=u32:?} }}",
            self.ctx1_aad0()
        )
    }
}
#[doc = "IPED Context1 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1aad1(pub u32);
impl Ipedctx1aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx1_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1aad1 {
    #[inline(always)]
    fn default() -> Ipedctx1aad1 {
        Ipedctx1aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx1aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1aad1")
            .field("ctx1_aad1", &self.ctx1_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1aad1 {{ ctx1_aad1: {=u32:?} }}",
            self.ctx1_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1end(pub u32);
impl Ipedctx1end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx1end {
    #[inline(always)]
    fn default() -> Ipedctx1end {
        Ipedctx1end(0)
    }
}
impl core::fmt::Debug for Ipedctx1end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context1 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1iv0(pub u32);
impl Ipedctx1iv0 {
    #[doc = "Lowest 32 bits of IV for region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 1."]
    #[inline(always)]
    pub const fn set_ctx1_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1iv0 {
    #[inline(always)]
    fn default() -> Ipedctx1iv0 {
        Ipedctx1iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx1iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1iv0")
            .field("ctx1_iv0", &self.ctx1_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx1iv0 {{ ctx1_iv0: {=u32:?} }}", self.ctx1_iv0())
    }
}
#[doc = "IPED Context1 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1iv1(pub u32);
impl Ipedctx1iv1 {
    #[doc = "Highest 32 bits of IV for region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 1."]
    #[inline(always)]
    pub const fn set_ctx1_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1iv1 {
    #[inline(always)]
    fn default() -> Ipedctx1iv1 {
        Ipedctx1iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx1iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1iv1")
            .field("ctx1_iv1", &self.ctx1_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx1iv1 {{ ctx1_iv1: {=u32:?} }}", self.ctx1_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1start(pub u32);
impl Ipedctx1start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx1startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx1startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx1startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx1startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx1startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx1startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx1start {
    #[inline(always)]
    fn default() -> Ipedctx1start {
        Ipedctx1start(0)
    }
}
impl core::fmt::Debug for Ipedctx1start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context2 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2aad0(pub u32);
impl Ipedctx2aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx2_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2aad0 {
    #[inline(always)]
    fn default() -> Ipedctx2aad0 {
        Ipedctx2aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx2aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2aad0")
            .field("ctx2_aad0", &self.ctx2_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2aad0 {{ ctx2_aad0: {=u32:?} }}",
            self.ctx2_aad0()
        )
    }
}
#[doc = "IPED Context2 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2aad1(pub u32);
impl Ipedctx2aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx2_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2aad1 {
    #[inline(always)]
    fn default() -> Ipedctx2aad1 {
        Ipedctx2aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx2aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2aad1")
            .field("ctx2_aad1", &self.ctx2_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2aad1 {{ ctx2_aad1: {=u32:?} }}",
            self.ctx2_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2end(pub u32);
impl Ipedctx2end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx2end {
    #[inline(always)]
    fn default() -> Ipedctx2end {
        Ipedctx2end(0)
    }
}
impl core::fmt::Debug for Ipedctx2end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context2 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2iv0(pub u32);
impl Ipedctx2iv0 {
    #[doc = "Lowest 32 bits of IV for region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 2."]
    #[inline(always)]
    pub const fn set_ctx2_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2iv0 {
    #[inline(always)]
    fn default() -> Ipedctx2iv0 {
        Ipedctx2iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx2iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2iv0")
            .field("ctx2_iv0", &self.ctx2_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx2iv0 {{ ctx2_iv0: {=u32:?} }}", self.ctx2_iv0())
    }
}
#[doc = "IPED Context2 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2iv1(pub u32);
impl Ipedctx2iv1 {
    #[doc = "Highest 32 bits of IV for region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 2."]
    #[inline(always)]
    pub const fn set_ctx2_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2iv1 {
    #[inline(always)]
    fn default() -> Ipedctx2iv1 {
        Ipedctx2iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx2iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2iv1")
            .field("ctx2_iv1", &self.ctx2_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx2iv1 {{ ctx2_iv1: {=u32:?} }}", self.ctx2_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2start(pub u32);
impl Ipedctx2start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx2startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx2startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx2startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx2startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx2startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx2startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx2start {
    #[inline(always)]
    fn default() -> Ipedctx2start {
        Ipedctx2start(0)
    }
}
impl core::fmt::Debug for Ipedctx2start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context3 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3aad0(pub u32);
impl Ipedctx3aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx3_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3aad0 {
    #[inline(always)]
    fn default() -> Ipedctx3aad0 {
        Ipedctx3aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx3aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3aad0")
            .field("ctx3_aad0", &self.ctx3_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3aad0 {{ ctx3_aad0: {=u32:?} }}",
            self.ctx3_aad0()
        )
    }
}
#[doc = "IPED Context3 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3aad1(pub u32);
impl Ipedctx3aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx3_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3aad1 {
    #[inline(always)]
    fn default() -> Ipedctx3aad1 {
        Ipedctx3aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx3aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3aad1")
            .field("ctx3_aad1", &self.ctx3_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3aad1 {{ ctx3_aad1: {=u32:?} }}",
            self.ctx3_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3end(pub u32);
impl Ipedctx3end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx3end {
    #[inline(always)]
    fn default() -> Ipedctx3end {
        Ipedctx3end(0)
    }
}
impl core::fmt::Debug for Ipedctx3end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context3 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3iv0(pub u32);
impl Ipedctx3iv0 {
    #[doc = "Lowest 32 bits of IV for region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 3."]
    #[inline(always)]
    pub const fn set_ctx3_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3iv0 {
    #[inline(always)]
    fn default() -> Ipedctx3iv0 {
        Ipedctx3iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx3iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3iv0")
            .field("ctx3_iv0", &self.ctx3_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx3iv0 {{ ctx3_iv0: {=u32:?} }}", self.ctx3_iv0())
    }
}
#[doc = "IPED Context3 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3iv1(pub u32);
impl Ipedctx3iv1 {
    #[doc = "Highest 32 bits of IV for region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 3."]
    #[inline(always)]
    pub const fn set_ctx3_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3iv1 {
    #[inline(always)]
    fn default() -> Ipedctx3iv1 {
        Ipedctx3iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx3iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3iv1")
            .field("ctx3_iv1", &self.ctx3_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx3iv1 {{ ctx3_iv1: {=u32:?} }}", self.ctx3_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3start(pub u32);
impl Ipedctx3start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx3startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx3startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx3startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx3startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx3startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx3startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx3start {
    #[inline(always)]
    fn default() -> Ipedctx3start {
        Ipedctx3start(0)
    }
}
impl core::fmt::Debug for Ipedctx3start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context4 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4aad0(pub u32);
impl Ipedctx4aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx4_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4aad0 {
    #[inline(always)]
    fn default() -> Ipedctx4aad0 {
        Ipedctx4aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx4aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4aad0")
            .field("ctx4_aad0", &self.ctx4_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4aad0 {{ ctx4_aad0: {=u32:?} }}",
            self.ctx4_aad0()
        )
    }
}
#[doc = "IPED Context4 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4aad1(pub u32);
impl Ipedctx4aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx4_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4aad1 {
    #[inline(always)]
    fn default() -> Ipedctx4aad1 {
        Ipedctx4aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx4aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4aad1")
            .field("ctx4_aad1", &self.ctx4_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4aad1 {{ ctx4_aad1: {=u32:?} }}",
            self.ctx4_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4end(pub u32);
impl Ipedctx4end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx4end {
    #[inline(always)]
    fn default() -> Ipedctx4end {
        Ipedctx4end(0)
    }
}
impl core::fmt::Debug for Ipedctx4end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context4 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4iv0(pub u32);
impl Ipedctx4iv0 {
    #[doc = "Lowest 32 bits of IV for region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 4."]
    #[inline(always)]
    pub const fn set_ctx4_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4iv0 {
    #[inline(always)]
    fn default() -> Ipedctx4iv0 {
        Ipedctx4iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx4iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4iv0")
            .field("ctx4_iv0", &self.ctx4_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx4iv0 {{ ctx4_iv0: {=u32:?} }}", self.ctx4_iv0())
    }
}
#[doc = "IPED Context4 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4iv1(pub u32);
impl Ipedctx4iv1 {
    #[doc = "Highest 32 bits of IV for region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 4."]
    #[inline(always)]
    pub const fn set_ctx4_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4iv1 {
    #[inline(always)]
    fn default() -> Ipedctx4iv1 {
        Ipedctx4iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx4iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4iv1")
            .field("ctx4_iv1", &self.ctx4_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx4iv1 {{ ctx4_iv1: {=u32:?} }}", self.ctx4_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4start(pub u32);
impl Ipedctx4start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx4startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx4startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx4startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx4startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx4startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx4startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx4start {
    #[inline(always)]
    fn default() -> Ipedctx4start {
        Ipedctx4start(0)
    }
}
impl core::fmt::Debug for Ipedctx4start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context5 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5aad0(pub u32);
impl Ipedctx5aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx5_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5aad0 {
    #[inline(always)]
    fn default() -> Ipedctx5aad0 {
        Ipedctx5aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx5aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5aad0")
            .field("ctx5_aad0", &self.ctx5_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5aad0 {{ ctx5_aad0: {=u32:?} }}",
            self.ctx5_aad0()
        )
    }
}
#[doc = "IPED Context5 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5aad1(pub u32);
impl Ipedctx5aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx5_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5aad1 {
    #[inline(always)]
    fn default() -> Ipedctx5aad1 {
        Ipedctx5aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx5aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5aad1")
            .field("ctx5_aad1", &self.ctx5_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5aad1 {{ ctx5_aad1: {=u32:?} }}",
            self.ctx5_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5end(pub u32);
impl Ipedctx5end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx5end {
    #[inline(always)]
    fn default() -> Ipedctx5end {
        Ipedctx5end(0)
    }
}
impl core::fmt::Debug for Ipedctx5end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context5 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5iv0(pub u32);
impl Ipedctx5iv0 {
    #[doc = "Lowest 32 bits of IV for region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 5."]
    #[inline(always)]
    pub const fn set_ctx5_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5iv0 {
    #[inline(always)]
    fn default() -> Ipedctx5iv0 {
        Ipedctx5iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx5iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5iv0")
            .field("ctx5_iv0", &self.ctx5_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx5iv0 {{ ctx5_iv0: {=u32:?} }}", self.ctx5_iv0())
    }
}
#[doc = "IPED Context5 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5iv1(pub u32);
impl Ipedctx5iv1 {
    #[doc = "Highest 32 bits of IV for region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 5."]
    #[inline(always)]
    pub const fn set_ctx5_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5iv1 {
    #[inline(always)]
    fn default() -> Ipedctx5iv1 {
        Ipedctx5iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx5iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5iv1")
            .field("ctx5_iv1", &self.ctx5_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx5iv1 {{ ctx5_iv1: {=u32:?} }}", self.ctx5_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5start(pub u32);
impl Ipedctx5start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx5startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx5startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx5startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx5startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx5startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx5startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx5start {
    #[inline(always)]
    fn default() -> Ipedctx5start {
        Ipedctx5start(0)
    }
}
impl core::fmt::Debug for Ipedctx5start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED Context6 Additional Authenticated Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6aad0(pub u32);
impl Ipedctx6aad0 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx6_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6aad0 {
    #[inline(always)]
    fn default() -> Ipedctx6aad0 {
        Ipedctx6aad0(0)
    }
}
impl core::fmt::Debug for Ipedctx6aad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6aad0")
            .field("ctx6_aad0", &self.ctx6_aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6aad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6aad0 {{ ctx6_aad0: {=u32:?} }}",
            self.ctx6_aad0()
        )
    }
}
#[doc = "IPED Context6 Additional Authenticated Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6aad1(pub u32);
impl Ipedctx6aad1 {
    #[doc = "CTX AAD"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD"]
    #[inline(always)]
    pub const fn set_ctx6_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6aad1 {
    #[inline(always)]
    fn default() -> Ipedctx6aad1 {
        Ipedctx6aad1(0)
    }
}
impl core::fmt::Debug for Ipedctx6aad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6aad1")
            .field("ctx6_aad1", &self.ctx6_aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6aad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6aad1 {{ ctx6_aad1: {=u32:?} }}",
            self.ctx6_aad1()
        )
    }
}
#[doc = "End Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6end(pub u32);
impl Ipedctx6end {
    #[doc = "End Address of IPED Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx6end {
    #[inline(always)]
    fn default() -> Ipedctx6end {
        Ipedctx6end(0)
    }
}
impl core::fmt::Debug for Ipedctx6end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context6 IV0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6iv0(pub u32);
impl Ipedctx6iv0 {
    #[doc = "Lowest 32 bits of IV for region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 6."]
    #[inline(always)]
    pub const fn set_ctx6_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6iv0 {
    #[inline(always)]
    fn default() -> Ipedctx6iv0 {
        Ipedctx6iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx6iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6iv0")
            .field("ctx6_iv0", &self.ctx6_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx6iv0 {{ ctx6_iv0: {=u32:?} }}", self.ctx6_iv0())
    }
}
#[doc = "IPED Context6 IV1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6iv1(pub u32);
impl Ipedctx6iv1 {
    #[doc = "Highest 32 bits of IV for region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 6."]
    #[inline(always)]
    pub const fn set_ctx6_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6iv1 {
    #[inline(always)]
    fn default() -> Ipedctx6iv1 {
        Ipedctx6iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx6iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6iv1")
            .field("ctx6_iv1", &self.ctx6_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx6iv1 {{ ctx6_iv1: {=u32:?} }}", self.ctx6_iv1())
    }
}
#[doc = "Start Address of Region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6start(pub u32);
impl Ipedctx6start {
    #[doc = "GCM Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> super::vals::Ipedctx6startGcm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipedctx6startGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable"]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: super::vals::Ipedctx6startGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> super::vals::Ipedctx6startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipedctx6startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable"]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: super::vals::Ipedctx6startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx6start {
    #[inline(always)]
    fn default() -> Ipedctx6start {
        Ipedctx6start(0)
    }
}
impl core::fmt::Debug for Ipedctx6start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6start")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6start {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED context control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl0(pub u32);
impl Ipedctxctrl0 {
    #[doc = "Context Register Freeze for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_freeze0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 0"]
    #[inline(always)]
    pub const fn set_ctx0_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Context Register Freeze for Region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_freeze0(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 1"]
    #[inline(always)]
    pub const fn set_ctx1_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Context Register Freeze for Region 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_freeze0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 2"]
    #[inline(always)]
    pub const fn set_ctx2_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Context Register Freeze for Region 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_freeze0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 3"]
    #[inline(always)]
    pub const fn set_ctx3_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Context Register Freeze for Region 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_freeze0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 4"]
    #[inline(always)]
    pub const fn set_ctx4_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Context Register Freeze for Region 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_freeze0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 5"]
    #[inline(always)]
    pub const fn set_ctx5_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Context Register Freeze for Region 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_freeze0(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 6"]
    #[inline(always)]
    pub const fn set_ctx6_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Ipedctxctrl0 {
    #[inline(always)]
    fn default() -> Ipedctxctrl0 {
        Ipedctxctrl0(0)
    }
}
impl core::fmt::Debug for Ipedctxctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxctrl0")
            .field("ctx0_freeze0", &self.ctx0_freeze0())
            .field("ctx1_freeze0", &self.ctx1_freeze0())
            .field("ctx2_freeze0", &self.ctx2_freeze0())
            .field("ctx3_freeze0", &self.ctx3_freeze0())
            .field("ctx4_freeze0", &self.ctx4_freeze0())
            .field("ctx5_freeze0", &self.ctx5_freeze0())
            .field("ctx6_freeze0", &self.ctx6_freeze0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl0 {{ ctx0_freeze0: {=u8:?}, ctx1_freeze0: {=u8:?}, ctx2_freeze0: {=u8:?}, ctx3_freeze0: {=u8:?}, ctx4_freeze0: {=u8:?}, ctx5_freeze0: {=u8:?}, ctx6_freeze0: {=u8:?} }}",
            self.ctx0_freeze0(),
            self.ctx1_freeze0(),
            self.ctx2_freeze0(),
            self.ctx3_freeze0(),
            self.ctx4_freeze0(),
            self.ctx5_freeze0(),
            self.ctx6_freeze0()
        )
    }
}
#[doc = "IPED context control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl1(pub u32);
impl Ipedctxctrl1 {
    #[doc = "Context Register Freeze for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_freeze1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 0"]
    #[inline(always)]
    pub const fn set_ctx0_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Context Register Freeze for Region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_freeze1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 1"]
    #[inline(always)]
    pub const fn set_ctx1_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Context Register Freeze for Region 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_freeze1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 2"]
    #[inline(always)]
    pub const fn set_ctx2_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Context Register Freeze for Region 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_freeze1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 3"]
    #[inline(always)]
    pub const fn set_ctx3_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Context Register Freeze for Region 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_freeze1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 4"]
    #[inline(always)]
    pub const fn set_ctx4_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Context Register Freeze for Region 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_freeze1(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 5"]
    #[inline(always)]
    pub const fn set_ctx5_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Context Register Freeze for Region 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_freeze1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 6"]
    #[inline(always)]
    pub const fn set_ctx6_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Ipedctxctrl1 {
    #[inline(always)]
    fn default() -> Ipedctxctrl1 {
        Ipedctxctrl1(0)
    }
}
impl core::fmt::Debug for Ipedctxctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxctrl1")
            .field("ctx0_freeze1", &self.ctx0_freeze1())
            .field("ctx1_freeze1", &self.ctx1_freeze1())
            .field("ctx2_freeze1", &self.ctx2_freeze1())
            .field("ctx3_freeze1", &self.ctx3_freeze1())
            .field("ctx4_freeze1", &self.ctx4_freeze1())
            .field("ctx5_freeze1", &self.ctx5_freeze1())
            .field("ctx6_freeze1", &self.ctx6_freeze1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl1 {{ ctx0_freeze1: {=u8:?}, ctx1_freeze1: {=u8:?}, ctx2_freeze1: {=u8:?}, ctx3_freeze1: {=u8:?}, ctx4_freeze1: {=u8:?}, ctx5_freeze1: {=u8:?}, ctx6_freeze1: {=u8:?} }}",
            self.ctx0_freeze1(),
            self.ctx1_freeze1(),
            self.ctx2_freeze1(),
            self.ctx3_freeze1(),
            self.ctx4_freeze1(),
            self.ctx5_freeze1(),
            self.ctx6_freeze1()
        )
    }
}
#[doc = "IP Receive FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfcr(pub u32);
impl Iprxfcr {
    #[doc = "Clear IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn clriprxf(&self) -> super::vals::Clriprxf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clriprxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_clriprxf(&mut self, val: super::vals::Clriprxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> super::vals::Rxdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxdmaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    #[inline(always)]
    pub const fn set_rxdmaen(&mut self, val: super::vals::Rxdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    #[inline(always)]
    pub const fn set_rxwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
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
            "Iprxfcr {{ clriprxf: {:?}, rxdmaen: {:?}, rxwmrk: {=u8:?} }}",
            self.clriprxf(),
            self.rxdmaen(),
            self.rxwmrk()
        )
    }
}
#[doc = "IP Receive FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfsts(pub u32);
impl Iprxfsts {
    #[doc = "Fill Level of IP Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Receive FIFO"]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read Data Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Data Counter"]
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
#[doc = "IPS Nonsecure Region 0 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszend0(pub u32);
impl Ipsnszend0 {
    #[doc = "End Address of Nonsecure Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Nonsecure Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszend0 {
    #[inline(always)]
    fn default() -> Ipsnszend0 {
        Ipsnszend0(0)
    }
}
impl core::fmt::Debug for Ipsnszend0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszend0")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszend0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszend0 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 1 End Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszend1(pub u32);
impl Ipsnszend1 {
    #[doc = "End Address of Nonsecure Region"]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Nonsecure Region"]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszend1 {
    #[inline(always)]
    fn default() -> Ipsnszend1 {
        Ipsnszend1(0)
    }
}
impl core::fmt::Debug for Ipsnszend1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszend1")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszend1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszend1 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 0 Start Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszstart0(pub u32);
impl Ipsnszstart0 {
    #[doc = "Start Address of Nonsecure Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Nonsecure Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszstart0 {
    #[inline(always)]
    fn default() -> Ipsnszstart0 {
        Ipsnszstart0(0)
    }
}
impl core::fmt::Debug for Ipsnszstart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszstart0")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszstart0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszstart0 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 1 Start Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszstart1(pub u32);
impl Ipsnszstart1 {
    #[doc = "Start Address of Nonsecure Region"]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Nonsecure Region"]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszstart1 {
    #[inline(always)]
    fn default() -> Ipsnszstart1 {
        Ipsnszstart1(0)
    }
}
impl core::fmt::Debug for Ipsnszstart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszstart1")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszstart1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszstart1 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "IP Transmit FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfcr(pub u32);
impl Iptxfcr {
    #[doc = "Clear IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn clriptxf(&self) -> super::vals::Clriptxf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clriptxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_clriptxf(&mut self, val: super::vals::Clriptxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txdmaen(&self) -> super::vals::Txdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Txdmaen::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO DMA Enable"]
    #[inline(always)]
    pub const fn set_txdmaen(&mut self, val: super::vals::Txdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Watermark Level"]
    #[must_use]
    #[inline(always)]
    pub const fn txwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit Watermark Level"]
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
            "Iptxfcr {{ clriptxf: {:?}, txdmaen: {:?}, txwmrk: {=u8:?} }}",
            self.clriptxf(),
            self.txdmaen(),
            self.txwmrk()
        )
    }
}
#[doc = "IP Transmit FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfsts(pub u32);
impl Iptxfsts {
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Transmit FIFO"]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Write Data Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn wrcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Data Counter"]
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
#[doc = "Lookup Table x"]
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
#[doc = "LUT Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutcr(pub u32);
impl Lutcr {
    #[doc = "Lock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::Lock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lock::from_bits(val as u8)
    }
    #[doc = "Lock LUT"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::Lock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Unlock LUT"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> super::vals::Unlock {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Unlock::from_bits(val as u8)
    }
    #[doc = "Unlock LUT"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: super::vals::Unlock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LUT Protection"]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::Protect {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Protect::from_bits(val as u8)
    }
    #[doc = "LUT Protection"]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: super::vals::Protect) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
            .field("protect", &self.protect())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lutcr {{ lock: {:?}, unlock: {:?}, protect: {:?} }}",
            self.lock(),
            self.unlock(),
            self.protect()
        )
    }
}
#[doc = "LUT Key"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutkey(pub u32);
impl Lutkey {
    #[doc = "LUT Key"]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LUT Key"]
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
#[doc = "Module Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr0(pub u32);
impl Mcr0 {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> super::vals::Swreset {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swreset::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: super::vals::Swreset) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: super::vals::Mdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc(&self) -> super::vals::Rxclksrc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rxclksrc::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    #[inline(always)]
    pub const fn set_rxclksrc(&mut self, val: super::vals::Rxclksrc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ardfen(&self) -> super::vals::Ardfen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ardfen::from_bits(val as u8)
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_ardfen(&mut self, val: super::vals::Ardfen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn atdfen(&self) -> super::vals::Atdfen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Atdfen::from_bits(val as u8)
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_atdfen(&mut self, val: super::vals::Atdfen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial Root Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn serclkdiv(&self) -> super::vals::Serclkdiv {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Serclkdiv::from_bits(val as u8)
    }
    #[doc = "Serial Root Clock Divider"]
    #[inline(always)]
    pub const fn set_serclkdiv(&mut self, val: super::vals::Serclkdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hsen(&self) -> super::vals::Hsen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hsen::from_bits(val as u8)
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    #[inline(always)]
    pub const fn set_hsen(&mut self, val: super::vals::Hsen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Combination Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn combinationen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Combination Mode Enable"]
    #[inline(always)]
    pub const fn set_combinationen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SCLK Free-running Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sckfreerunen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Free-running Enable"]
    #[inline(always)]
    pub const fn set_sckfreerunen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Learning Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn learnen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning Enable"]
    #[inline(always)]
    pub const fn set_learnen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgrantwait(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    #[inline(always)]
    pub const fn set_ipgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgrantwait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
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
            .field("ardfen", &self.ardfen())
            .field("atdfen", &self.atdfen())
            .field("serclkdiv", &self.serclkdiv())
            .field("hsen", &self.hsen())
            .field("dozeen", &self.dozeen())
            .field("combinationen", &self.combinationen())
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
            "Mcr0 {{ swreset: {:?}, mdis: {:?}, rxclksrc: {:?}, ardfen: {:?}, atdfen: {:?}, serclkdiv: {:?}, hsen: {:?}, dozeen: {:?}, combinationen: {=bool:?}, sckfreerunen: {=bool:?}, learnen: {=bool:?}, ipgrantwait: {=u8:?}, ahbgrantwait: {=u8:?} }}",
            self.swreset(),
            self.mdis(),
            self.rxclksrc(),
            self.ardfen(),
            self.atdfen(),
            self.serclkdiv(),
            self.hsen(),
            self.dozeen(),
            self.combinationen(),
            self.sckfreerunen(),
            self.learnen(),
            self.ipgrantwait(),
            self.ahbgrantwait()
        )
    }
}
#[doc = "Module Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr1(pub u32);
impl Mcr1 {
    #[doc = "AHB Bus Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuswait(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Bus Wait"]
    #[inline(always)]
    pub const fn set_ahbbuswait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Command Sequence Wait"]
    #[must_use]
    #[inline(always)]
    pub const fn seqwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Command Sequence Wait"]
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
#[doc = "Module Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr2(pub u32);
impl Mcr2 {
    #[doc = "Clear AHB Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbbufopt(&self) -> super::vals::Clrahbbufopt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrahbbufopt::from_bits(val as u8)
    }
    #[doc = "Clear AHB Buffer"]
    #[inline(always)]
    pub const fn set_clrahbbufopt(&mut self, val: super::vals::Clrahbbufopt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Clear Learn Phase Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn clrlearnphase(&self) -> super::vals::Clrlearnphase {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Clrlearnphase::from_bits(val as u8)
    }
    #[doc = "Clear Learn Phase Selection"]
    #[inline(always)]
    pub const fn set_clrlearnphase(&mut self, val: super::vals::Clrlearnphase) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Same Device Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn samedeviceen(&self) -> super::vals::Samedeviceen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Samedeviceen::from_bits(val as u8)
    }
    #[doc = "Same Device Enable"]
    #[inline(always)]
    pub const fn set_samedeviceen(&mut self, val: super::vals::Samedeviceen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "SCLK Port B Differential Output"]
    #[must_use]
    #[inline(always)]
    pub const fn sckbdiffopt(&self) -> super::vals::Sckbdiffopt {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sckbdiffopt::from_bits(val as u8)
    }
    #[doc = "SCLK Port B Differential Output"]
    #[inline(always)]
    pub const fn set_sckbdiffopt(&mut self, val: super::vals::Sckbdiffopt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Port B Receiver Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc_b(&self) -> super::vals::RxclksrcB {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::RxclksrcB::from_bits(val as u8)
    }
    #[doc = "Port B Receiver Clock Source"]
    #[inline(always)]
    pub const fn set_rxclksrc_b(&mut self, val: super::vals::RxclksrcB) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Sample Clock Source Different"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_clk_src_diff(&self) -> super::vals::RxClkSrcDiff {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RxClkSrcDiff::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source Different"]
    #[inline(always)]
    pub const fn set_rx_clk_src_diff(&mut self, val: super::vals::RxClkSrcDiff) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Resume Wait Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn resumewait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Resume Wait Duration"]
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
            .field("rxclksrc_b", &self.rxclksrc_b())
            .field("rx_clk_src_diff", &self.rx_clk_src_diff())
            .field("resumewait", &self.resumewait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr2 {{ clrahbbufopt: {:?}, clrlearnphase: {:?}, samedeviceen: {:?}, sckbdiffopt: {:?}, rxclksrc_b: {:?}, rx_clk_src_diff: {:?}, resumewait: {=u8:?} }}",
            self.clrahbbufopt(),
            self.clrlearnphase(),
            self.samedeviceen(),
            self.sckbdiffopt(),
            self.rxclksrc_b(),
            self.rx_clk_src_diff(),
            self.resumewait()
        )
    }
}
#[doc = "IP Receive FIFO Data x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfdr(pub u32);
impl Rfdr {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
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
#[doc = "Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "SEQ_CTL State Machine Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn seqidle(&self) -> super::vals::Seqidle {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Seqidle::from_bits(val as u8)
    }
    #[doc = "SEQ_CTL State Machine Idle"]
    #[inline(always)]
    pub const fn set_seqidle(&mut self, val: super::vals::Seqidle) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ARB_CTL State Machine Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn arbidle(&self) -> super::vals::Arbidle {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Arbidle::from_bits(val as u8)
    }
    #[doc = "ARB_CTL State Machine Idle"]
    #[inline(always)]
    pub const fn set_arbidle(&mut self, val: super::vals::Arbidle) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ARB Command Source"]
    #[must_use]
    #[inline(always)]
    pub const fn arbcmdsrc(&self) -> super::vals::Arbcmdsrc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Arbcmdsrc::from_bits(val as u8)
    }
    #[doc = "ARB Command Source"]
    #[inline(always)]
    pub const fn set_arbcmdsrc(&mut self, val: super::vals::Arbcmdsrc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Data Learning Phase Selection on Port A"]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphasea(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Learning Phase Selection on Port A"]
    #[inline(always)]
    pub const fn set_datalearnphasea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data Learning Phase Selection on Port B"]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphaseb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Learning Phase Selection on Port B"]
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
            "Sts0 {{ seqidle: {:?}, arbidle: {:?}, arbcmdsrc: {:?}, datalearnphasea: {=u8:?}, datalearnphaseb: {=u8:?} }}",
            self.seqidle(),
            self.arbidle(),
            self.arbcmdsrc(),
            self.datalearnphasea(),
            self.datalearnphaseb()
        )
    }
}
#[doc = "Status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts1(pub u32);
impl Sts1 {
    #[doc = "AHB Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Command Error ID"]
    #[inline(always)]
    pub const fn set_ahbcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "AHB Command Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrcode(&self) -> super::vals::Ahbcmderrcode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ahbcmderrcode::from_bits(val as u8)
    }
    #[doc = "AHB Command Error Code"]
    #[inline(always)]
    pub const fn set_ahbcmderrcode(&mut self, val: super::vals::Ahbcmderrcode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "IP Command Error ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IP Command Error ID"]
    #[inline(always)]
    pub const fn set_ipcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "IP Command Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrcode(&self) -> super::vals::Ipcmderrcode {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ipcmderrcode::from_bits(val as u8)
    }
    #[doc = "IP Command Error Code"]
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
#[doc = "Status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "Flash A Sample Target Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn aslvlock(&self) -> super::vals::Aslvlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aslvlock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Target Delay Line Locked"]
    #[inline(always)]
    pub const fn set_aslvlock(&mut self, val: super::vals::Aslvlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn areflock(&self) -> super::vals::Areflock {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Areflock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_areflock(&mut self, val: super::vals::Areflock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn aslvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_aslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn arefsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_arefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn bslvlock(&self) -> super::vals::Bslvlock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Bslvlock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_bslvlock(&mut self, val: super::vals::Bslvlock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    #[must_use]
    #[inline(always)]
    pub const fn breflock(&self) -> super::vals::Breflock {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Breflock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    #[inline(always)]
    pub const fn set_breflock(&mut self, val: super::vals::Breflock) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn bslvsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    #[inline(always)]
    pub const fn set_bslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
    #[must_use]
    #[inline(always)]
    pub const fn brefsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
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
            "Sts2 {{ aslvlock: {:?}, areflock: {:?}, aslvsel: {=u8:?}, arefsel: {=u8:?}, bslvlock: {:?}, breflock: {:?}, bslvsel: {=u8:?}, brefsel: {=u8:?} }}",
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
#[doc = "IP TX FIFO Data x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfdr(pub u32);
impl Tfdr {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data"]
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
