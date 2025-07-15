#[doc = "Block Initial Vector for Memory Context n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BivctxWd(pub u32);
impl BivctxWd {
    #[doc = "Block Initial Vector Word0"]
    #[must_use]
    #[inline(always)]
    pub const fn biv_wd0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Block Initial Vector Word0"]
    #[inline(always)]
    pub const fn set_biv_wd0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BivctxWd {
    #[inline(always)]
    fn default() -> BivctxWd {
        BivctxWd(0)
    }
}
impl core::fmt::Debug for BivctxWd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BivctxWd")
            .field("biv_wd0", &self.biv_wd0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BivctxWd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BivctxWd {{ biv_wd0: {=u32:?} }}", self.biv_wd0())
    }
}
#[doc = "Flash Cache Obfuscation Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacmsk(pub u32);
impl Cacmsk {
    #[doc = "Obfuscation Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn obmask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Obfuscation Mask"]
    #[inline(always)]
    pub const fn set_obmask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cacmsk {
    #[inline(always)]
    fn default() -> Cacmsk {
        Cacmsk(0)
    }
}
impl core::fmt::Debug for Cacmsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacmsk")
            .field("obmask", &self.obmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacmsk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cacmsk {{ obmask: {=u32:?} }}", self.obmask())
    }
}
#[doc = "NPX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npxcr(pub u32);
impl Npxcr {
    #[doc = "Global Encryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gee(&self) -> super::vals::Gee {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gee::from_bits(val as u8)
    }
    #[doc = "Global Encryption Enable"]
    #[inline(always)]
    pub const fn set_gee(&mut self, val: super::vals::Gee) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Decryption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gde(&self) -> super::vals::Gde {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Gde::from_bits(val as u8)
    }
    #[doc = "Global Decryption Enable"]
    #[inline(always)]
    pub const fn set_gde(&mut self, val: super::vals::Gde) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Lock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn glk(&self) -> super::vals::Glk {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Glk::from_bits(val as u8)
    }
    #[doc = "Global Lock Enable"]
    #[inline(always)]
    pub const fn set_glk(&mut self, val: super::vals::Glk) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask Lock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mlk(&self) -> super::vals::Mlk {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mlk::from_bits(val as u8)
    }
    #[doc = "Mask Lock Enable"]
    #[inline(always)]
    pub const fn set_mlk(&mut self, val: super::vals::Mlk) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock Enable for Context 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0lk(&self) -> super::vals::Ctx0lk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ctx0lk::from_bits(val as u8)
    }
    #[doc = "Lock Enable for Context 0"]
    #[inline(always)]
    pub const fn set_ctx0lk(&mut self, val: super::vals::Ctx0lk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Lock Enable for Context 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1lk(&self) -> super::vals::Ctx1lk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ctx1lk::from_bits(val as u8)
    }
    #[doc = "Lock Enable for Context 1"]
    #[inline(always)]
    pub const fn set_ctx1lk(&mut self, val: super::vals::Ctx1lk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Lock Enable for Context 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2lk(&self) -> super::vals::Ctx2lk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ctx2lk::from_bits(val as u8)
    }
    #[doc = "Lock Enable for Context 2"]
    #[inline(always)]
    pub const fn set_ctx2lk(&mut self, val: super::vals::Ctx2lk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Lock Enable for Context 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3lk(&self) -> super::vals::Ctx3lk {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ctx3lk::from_bits(val as u8)
    }
    #[doc = "Lock Enable for Context 3"]
    #[inline(always)]
    pub const fn set_ctx3lk(&mut self, val: super::vals::Ctx3lk) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for Npxcr {
    #[inline(always)]
    fn default() -> Npxcr {
        Npxcr(0)
    }
}
impl core::fmt::Debug for Npxcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Npxcr")
            .field("gee", &self.gee())
            .field("gde", &self.gde())
            .field("glk", &self.glk())
            .field("mlk", &self.mlk())
            .field("ctx0lk", &self.ctx0lk())
            .field("ctx1lk", &self.ctx1lk())
            .field("ctx2lk", &self.ctx2lk())
            .field("ctx3lk", &self.ctx3lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Npxcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Npxcr {{ gee: {:?}, gde: {:?}, glk: {:?}, mlk: {:?}, ctx0lk: {:?}, ctx1lk: {:?}, ctx2lk: {:?}, ctx3lk: {:?} }}",
            self.gee(),
            self.gde(),
            self.glk(),
            self.mlk(),
            self.ctx0lk(),
            self.ctx1lk(),
            self.ctx2lk(),
            self.ctx3lk()
        )
    }
}
#[doc = "NPX Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npxsr(pub u32);
impl Npxsr {
    #[doc = "Number of implemented memory contexts"]
    #[must_use]
    #[inline(always)]
    pub const fn numctx(&self) -> super::vals::Numctx {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Numctx::from_bits(val as u8)
    }
    #[doc = "Number of implemented memory contexts"]
    #[inline(always)]
    pub const fn set_numctx(&mut self, val: super::vals::Numctx) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Key n Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v0(&self) -> super::vals::V0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::V0::from_bits(val as u8)
    }
    #[doc = "Key n Valid"]
    #[inline(always)]
    pub const fn set_v0(&mut self, val: super::vals::V0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Key n Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v1(&self) -> super::vals::V1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::V1::from_bits(val as u8)
    }
    #[doc = "Key n Valid"]
    #[inline(always)]
    pub const fn set_v1(&mut self, val: super::vals::V1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Key n Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v2(&self) -> super::vals::V2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::V2::from_bits(val as u8)
    }
    #[doc = "Key n Valid"]
    #[inline(always)]
    pub const fn set_v2(&mut self, val: super::vals::V2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Key n Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v3(&self) -> super::vals::V3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::V3::from_bits(val as u8)
    }
    #[doc = "Key n Valid"]
    #[inline(always)]
    pub const fn set_v3(&mut self, val: super::vals::V3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Npxsr {
    #[inline(always)]
    fn default() -> Npxsr {
        Npxsr(0)
    }
}
impl core::fmt::Debug for Npxsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Npxsr")
            .field("numctx", &self.numctx())
            .field("v0", &self.v0())
            .field("v1", &self.v1())
            .field("v2", &self.v2())
            .field("v3", &self.v3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Npxsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Npxsr {{ numctx: {:?}, v0: {:?}, v1: {:?}, v2: {:?}, v3: {:?} }}",
            self.numctx(),
            self.v0(),
            self.v1(),
            self.v2(),
            self.v3()
        )
    }
}
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
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "LIM Remapping Address"]
    #[inline(always)]
    pub const fn set_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LIMDP Remapping Address"]
    #[must_use]
    #[inline(always)]
    pub const fn limdp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "LIMDP Remapping Address"]
    #[inline(always)]
    pub const fn set_limdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
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
#[doc = "Bitmap of Valid Control for Memory Context n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmapctxWd(pub u32);
impl VmapctxWd {
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[must_use]
    #[inline(always)]
    pub const fn val31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption"]
    #[inline(always)]
    pub const fn set_val31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VmapctxWd {
    #[inline(always)]
    fn default() -> VmapctxWd {
        VmapctxWd(0)
    }
}
impl core::fmt::Debug for VmapctxWd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VmapctxWd")
            .field("val0", &self.val0())
            .field("val1", &self.val1())
            .field("val2", &self.val2())
            .field("val3", &self.val3())
            .field("val4", &self.val4())
            .field("val5", &self.val5())
            .field("val6", &self.val6())
            .field("val7", &self.val7())
            .field("val8", &self.val8())
            .field("val9", &self.val9())
            .field("val10", &self.val10())
            .field("val11", &self.val11())
            .field("val12", &self.val12())
            .field("val13", &self.val13())
            .field("val14", &self.val14())
            .field("val15", &self.val15())
            .field("val16", &self.val16())
            .field("val17", &self.val17())
            .field("val18", &self.val18())
            .field("val19", &self.val19())
            .field("val20", &self.val20())
            .field("val21", &self.val21())
            .field("val22", &self.val22())
            .field("val23", &self.val23())
            .field("val24", &self.val24())
            .field("val25", &self.val25())
            .field("val26", &self.val26())
            .field("val27", &self.val27())
            .field("val28", &self.val28())
            .field("val29", &self.val29())
            .field("val30", &self.val30())
            .field("val31", &self.val31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VmapctxWd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VmapctxWd {{ val0: {=bool:?}, val1: {=bool:?}, val2: {=bool:?}, val3: {=bool:?}, val4: {=bool:?}, val5: {=bool:?}, val6: {=bool:?}, val7: {=bool:?}, val8: {=bool:?}, val9: {=bool:?}, val10: {=bool:?}, val11: {=bool:?}, val12: {=bool:?}, val13: {=bool:?}, val14: {=bool:?}, val15: {=bool:?}, val16: {=bool:?}, val17: {=bool:?}, val18: {=bool:?}, val19: {=bool:?}, val20: {=bool:?}, val21: {=bool:?}, val22: {=bool:?}, val23: {=bool:?}, val24: {=bool:?}, val25: {=bool:?}, val26: {=bool:?}, val27: {=bool:?}, val28: {=bool:?}, val29: {=bool:?}, val30: {=bool:?}, val31: {=bool:?} }}",
            self.val0(),
            self.val1(),
            self.val2(),
            self.val3(),
            self.val4(),
            self.val5(),
            self.val6(),
            self.val7(),
            self.val8(),
            self.val9(),
            self.val10(),
            self.val11(),
            self.val12(),
            self.val13(),
            self.val14(),
            self.val15(),
            self.val16(),
            self.val17(),
            self.val18(),
            self.val19(),
            self.val20(),
            self.val21(),
            self.val22(),
            self.val23(),
            self.val24(),
            self.val25(),
            self.val26(),
            self.val27(),
            self.val28(),
            self.val29(),
            self.val30(),
            self.val31()
        )
    }
}
