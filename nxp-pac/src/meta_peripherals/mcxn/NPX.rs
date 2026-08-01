#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: VMAPCTX_WD%s, BIVCTX_WD%s."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxValidIvArray {
    ptr: *mut u8,
}
unsafe impl Send for CtxValidIvArray {}
unsafe impl Sync for CtxValidIvArray {}
impl CtxValidIvArray {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bitmap of Valid Control for Memory Context n."]
    #[inline(always)]
    pub const fn vmapctx_wd(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<VmapctxWd, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
        }
    }
    #[doc = "Block Initial Vector for Memory Context n."]
    #[inline(always)]
    pub const fn bivctx_wd(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<BivctxWd, crate::pac::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _)
        }
    }
}
#[doc = "NPX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npx {
    ptr: *mut u8,
}
unsafe impl Send for Npx {}
unsafe impl Sync for Npx {}
impl Npx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NPX Control Register."]
    #[inline(always)]
    pub const fn npxcr(self) -> crate::pac::common::Reg<Npxcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "NPX Status Register."]
    #[inline(always)]
    pub const fn npxsr(self) -> crate::pac::common::Reg<Npxsr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Cache Obfuscation Mask."]
    #[inline(always)]
    pub const fn cacmsk(self) -> crate::pac::common::Reg<Cacmsk, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data Remap."]
    #[inline(always)]
    pub const fn remap(self) -> crate::pac::common::Reg<Remap, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Array of registers: VMAPCTX_WD%s, BIVCTX_WD%s."]
    #[inline(always)]
    pub const fn ctx_valid_iv_array(self, n: usize) -> CtxValidIvArray {
        assert!(n < 4usize);
        unsafe { CtxValidIvArray::from_ptr(self.ptr.wrapping_add(0x40usize + n * 16usize) as _) }
    }
}
#[doc = "Block Initial Vector for Memory Context n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BivctxWd(pub u32);
impl BivctxWd {
    #[doc = "Block Initial Vector Word0."]
    #[must_use]
    #[inline(always)]
    pub const fn biv_wd0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Block Initial Vector Word0."]
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
#[doc = "Flash Cache Obfuscation Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacmsk(pub u32);
impl Cacmsk {
    #[doc = "Obfuscation Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn obmask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Obfuscation Mask."]
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
#[doc = "NPX Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npxcr(pub u32);
impl Npxcr {
    #[doc = "Global Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gee(&self) -> Gee {
        let val = (self.0 >> 0usize) & 0x01;
        Gee::from_bits(val as u8)
    }
    #[doc = "Global Encryption Enable."]
    #[inline(always)]
    pub const fn set_gee(&mut self, val: Gee) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Global Decryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gde(&self) -> Gde {
        let val = (self.0 >> 2usize) & 0x01;
        Gde::from_bits(val as u8)
    }
    #[doc = "Global Decryption Enable."]
    #[inline(always)]
    pub const fn set_gde(&mut self, val: Gde) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Global Lock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn glk(&self) -> Glk {
        let val = (self.0 >> 4usize) & 0x01;
        Glk::from_bits(val as u8)
    }
    #[doc = "Global Lock Enable."]
    #[inline(always)]
    pub const fn set_glk(&mut self, val: Glk) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask Lock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mlk(&self) -> Mlk {
        let val = (self.0 >> 6usize) & 0x01;
        Mlk::from_bits(val as u8)
    }
    #[doc = "Mask Lock Enable."]
    #[inline(always)]
    pub const fn set_mlk(&mut self, val: Mlk) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Lock Enable for Context n."]
    #[must_use]
    #[inline(always)]
    pub const fn ctxlk(&self, n: usize) -> Ctxlk {
        assert!(n < 4usize);
        let offs = 8usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        Ctxlk::from_bits(val as u8)
    }
    #[doc = "Lock Enable for Context n."]
    #[inline(always)]
    pub const fn set_ctxlk(&mut self, n: usize, val: Ctxlk) {
        assert!(n < 4usize);
        let offs = 8usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
            .field("ctxlk[0]", &self.ctxlk(0usize))
            .field("ctxlk[1]", &self.ctxlk(1usize))
            .field("ctxlk[2]", &self.ctxlk(2usize))
            .field("ctxlk[3]", &self.ctxlk(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Npxcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Npxcr {{ gee: {:?}, gde: {:?}, glk: {:?}, mlk: {:?}, ctxlk[0]: {:?}, ctxlk[1]: {:?}, ctxlk[2]: {:?}, ctxlk[3]: {:?} }}",
            self.gee(),
            self.gde(),
            self.glk(),
            self.mlk(),
            self.ctxlk(0usize),
            self.ctxlk(1usize),
            self.ctxlk(2usize),
            self.ctxlk(3usize)
        )
    }
}
#[doc = "NPX Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npxsr(pub u32);
impl Npxsr {
    #[doc = "Number of implemented memory contexts."]
    #[must_use]
    #[inline(always)]
    pub const fn numctx(&self) -> Numctx {
        let val = (self.0 >> 0usize) & 0x0f;
        Numctx::from_bits(val as u8)
    }
    #[doc = "Number of implemented memory contexts."]
    #[inline(always)]
    pub const fn set_numctx(&mut self, val: Numctx) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Key n Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self, n: usize) -> V {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        V::from_bits(val as u8)
    }
    #[doc = "Key n Valid."]
    #[inline(always)]
    pub const fn set_v(&mut self, n: usize, val: V) {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
            .field("v[0]", &self.v(0usize))
            .field("v[1]", &self.v(1usize))
            .field("v[2]", &self.v(2usize))
            .field("v[3]", &self.v(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Npxsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Npxsr {{ numctx: {:?}, v[0]: {:?}, v[1]: {:?}, v[2]: {:?}, v[3]: {:?} }}",
            self.numctx(),
            self.v(0usize),
            self.v(1usize),
            self.v(2usize),
            self.v(3usize)
        )
    }
}
#[doc = "Data Remap."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "Remap Lock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn remaplk(&self) -> Remaplk {
        let val = (self.0 >> 0usize) & 0x01;
        Remaplk::from_bits(val as u8)
    }
    #[doc = "Remap Lock Enable."]
    #[inline(always)]
    pub const fn set_remaplk(&mut self, val: Remaplk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LIM Remapping Address."]
    #[must_use]
    #[inline(always)]
    pub const fn lim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "LIM Remapping Address."]
    #[inline(always)]
    pub const fn set_lim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LIMDP Remapping Address."]
    #[must_use]
    #[inline(always)]
    pub const fn limdp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "LIMDP Remapping Address."]
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
#[doc = "Bitmap of Valid Control for Memory Context n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VmapctxWd(pub u32);
impl VmapctxWd {
    #[doc = "Block valid enable for encryption/decryption."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Block valid enable for encryption/decryption."]
    #[inline(always)]
    pub const fn set_val(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("val[0]", &self.val(0usize))
            .field("val[1]", &self.val(1usize))
            .field("val[2]", &self.val(2usize))
            .field("val[3]", &self.val(3usize))
            .field("val[4]", &self.val(4usize))
            .field("val[5]", &self.val(5usize))
            .field("val[6]", &self.val(6usize))
            .field("val[7]", &self.val(7usize))
            .field("val[8]", &self.val(8usize))
            .field("val[9]", &self.val(9usize))
            .field("val[10]", &self.val(10usize))
            .field("val[11]", &self.val(11usize))
            .field("val[12]", &self.val(12usize))
            .field("val[13]", &self.val(13usize))
            .field("val[14]", &self.val(14usize))
            .field("val[15]", &self.val(15usize))
            .field("val[16]", &self.val(16usize))
            .field("val[17]", &self.val(17usize))
            .field("val[18]", &self.val(18usize))
            .field("val[19]", &self.val(19usize))
            .field("val[20]", &self.val(20usize))
            .field("val[21]", &self.val(21usize))
            .field("val[22]", &self.val(22usize))
            .field("val[23]", &self.val(23usize))
            .field("val[24]", &self.val(24usize))
            .field("val[25]", &self.val(25usize))
            .field("val[26]", &self.val(26usize))
            .field("val[27]", &self.val(27usize))
            .field("val[28]", &self.val(28usize))
            .field("val[29]", &self.val(29usize))
            .field("val[30]", &self.val(30usize))
            .field("val[31]", &self.val(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VmapctxWd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VmapctxWd {{ val[0]: {=bool:?}, val[1]: {=bool:?}, val[2]: {=bool:?}, val[3]: {=bool:?}, val[4]: {=bool:?}, val[5]: {=bool:?}, val[6]: {=bool:?}, val[7]: {=bool:?}, val[8]: {=bool:?}, val[9]: {=bool:?}, val[10]: {=bool:?}, val[11]: {=bool:?}, val[12]: {=bool:?}, val[13]: {=bool:?}, val[14]: {=bool:?}, val[15]: {=bool:?}, val[16]: {=bool:?}, val[17]: {=bool:?}, val[18]: {=bool:?}, val[19]: {=bool:?}, val[20]: {=bool:?}, val[21]: {=bool:?}, val[22]: {=bool:?}, val[23]: {=bool:?}, val[24]: {=bool:?}, val[25]: {=bool:?}, val[26]: {=bool:?}, val[27]: {=bool:?}, val[28]: {=bool:?}, val[29]: {=bool:?}, val[30]: {=bool:?}, val[31]: {=bool:?} }}",
            self.val(0usize),
            self.val(1usize),
            self.val(2usize),
            self.val(3usize),
            self.val(4usize),
            self.val(5usize),
            self.val(6usize),
            self.val(7usize),
            self.val(8usize),
            self.val(9usize),
            self.val(10usize),
            self.val(11usize),
            self.val(12usize),
            self.val(13usize),
            self.val(14usize),
            self.val(15usize),
            self.val(16usize),
            self.val(17usize),
            self.val(18usize),
            self.val(19usize),
            self.val(20usize),
            self.val(21usize),
            self.val(22usize),
            self.val(23usize),
            self.val(24usize),
            self.val(25usize),
            self.val(26usize),
            self.val(27usize),
            self.val(28usize),
            self.val(29usize),
            self.val(30usize),
            self.val(31usize)
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctxlk {
    #[doc = "Lock disabled: VMAPCTXn remains read-write."]
    LockDisabled = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTXn (becomes read-only)."]
    LockEnabled = 0x01,
}
impl Ctxlk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctxlk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctxlk {
    #[inline(always)]
    fn from(val: u8) -> Ctxlk {
        Ctxlk::from_bits(val)
    }
}
impl From<Ctxlk> for u8 {
    #[inline(always)]
    fn from(val: Ctxlk) -> u8 {
        Ctxlk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gde {
    #[doc = "Global decryption disabled. NPX on-the-fly decryption is globally disabled. Subsequent reads return 0."]
    DecryptionDisabled = 0x0,
    #[doc = "Global decryption enabled. NPX on-the-fly decryption is globally enabled. Subsequent reads return 1."]
    DecryptionEnabled = 0x01,
}
impl Gde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gde {
    #[inline(always)]
    fn from(val: u8) -> Gde {
        Gde::from_bits(val)
    }
}
impl From<Gde> for u8 {
    #[inline(always)]
    fn from(val: Gde) -> u8 {
        Gde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gee {
    #[doc = "Global encryption disabled. NPX on-the-fly encryption is disabled. Subsequent reads return 0."]
    EncryptionDisabled = 0x0,
    #[doc = "Global encryption enabled. NPX on-the-fly encryption is enabled if the flash access hits in a valid memory context. Subsequent reads return 1."]
    EncryptionEnabled = 0x01,
}
impl Gee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gee {
    #[inline(always)]
    fn from(val: u8) -> Gee {
        Gee::from_bits(val)
    }
}
impl From<Gee> for u8 {
    #[inline(always)]
    fn from(val: Gee) -> u8 {
        Gee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Glk {
    #[doc = "Lock disabled. Subsequent reads return 0."]
    LockDisabled = 0x0,
    #[doc = "Lock enabled: cannot write to VMAPCTXn, NPXCR, or CACMSK. Subsequent reads return 1."]
    LockEnabled = 0x01,
}
impl Glk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glk {
    #[inline(always)]
    fn from(val: u8) -> Glk {
        Glk::from_bits(val)
    }
}
impl From<Glk> for u8 {
    #[inline(always)]
    fn from(val: Glk) -> u8 {
        Glk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mlk {
    #[doc = "Lock disabled. Subsequent reads return 0."]
    LockDisabled = 0x0,
    #[doc = "Lock enabled: cannot write to mask. Subsequent reads return 1."]
    LockEnabled = 0x01,
}
impl Mlk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mlk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mlk {
    #[inline(always)]
    fn from(val: u8) -> Mlk {
        Mlk::from_bits(val)
    }
}
impl From<Mlk> for u8 {
    #[inline(always)]
    fn from(val: Mlk) -> u8 {
        Mlk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Numctx {
    #[doc = "No (zero) implemented memory contexts."]
    ZeroCtx = 0x0,
    #[doc = "1 implemented memory contexts."]
    OneCtx = 0x01,
    #[doc = "2 implemented memory contexts."]
    TwoCtx = 0x02,
    #[doc = "3 implemented memory contexts."]
    ThreeCtx = 0x03,
    #[doc = "4 implemented memory contexts."]
    FourCtx = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Numctx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Numctx {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Numctx {
    #[inline(always)]
    fn from(val: u8) -> Numctx {
        Numctx::from_bits(val)
    }
}
impl From<Numctx> for u8 {
    #[inline(always)]
    fn from(val: Numctx) -> u8 {
        Numctx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remaplk {
    #[doc = "Lock disabled: can write to REMAP."]
    LockDisabled = 0x0,
    #[doc = "Lock enabled: cannot write to REMAP."]
    LockEnabled = 0x01,
}
impl Remaplk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remaplk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remaplk {
    #[inline(always)]
    fn from(val: u8) -> Remaplk {
        Remaplk::from_bits(val)
    }
}
impl From<Remaplk> for u8 {
    #[inline(always)]
    fn from(val: Remaplk) -> u8 {
        Remaplk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum V {
    #[doc = "Not valid."]
    KeyNotvalid = 0x0,
    #[doc = "Valid."]
    KeyValid = 0x01,
}
impl V {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> V {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for V {
    #[inline(always)]
    fn from(val: u8) -> V {
        V::from_bits(val)
    }
}
impl From<V> for u8 {
    #[inline(always)]
    fn from(val: V) -> u8 {
        V::to_bits(val)
    }
}
