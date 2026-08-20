#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "TRDC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trdc {
    ptr: *mut u8,
}
unsafe impl Send for Trdc {}
unsafe impl Sync for Trdc {}
impl Trdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MBC Global Configuration Register."]
    #[inline(always)]
    pub const fn mbc0_mem0_glbcfg(
        self,
    ) -> crate::pac::common::Reg<Mbc0Mem0Glbcfg, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MBC Global Configuration Register."]
    #[inline(always)]
    pub const fn mbc0_mem1_glbcfg(
        self,
    ) -> crate::pac::common::Reg<Mbc0Mem1Glbcfg, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MBC Global Configuration Register."]
    #[inline(always)]
    pub const fn mbc0_mem2_glbcfg(
        self,
    ) -> crate::pac::common::Reg<Mbc0Mem2Glbcfg, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MBC Global Configuration Register."]
    #[inline(always)]
    pub const fn mbc0_mem3_glbcfg(
        self,
    ) -> crate::pac::common::Reg<Mbc0Mem3Glbcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Index."]
    #[inline(always)]
    pub const fn mbc0_nse_blk_index(
        self,
    ) -> crate::pac::common::Reg<Mbc0NseBlkIndex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Set."]
    #[inline(always)]
    pub const fn mbc0_nse_blk_set(
        self,
    ) -> crate::pac::common::Reg<Mbc0NseBlkSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear."]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr(
        self,
    ) -> crate::pac::common::Reg<Mbc0NseBlkClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear All."]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr_all(
        self,
    ) -> crate::pac::common::Reg<Mbc0NseBlkClrAll, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac0(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac1(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac2(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac3(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac4(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac5(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac6(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MBC Global Access Control."]
    #[inline(always)]
    pub const fn mbc0_memn_glbac7(
        self,
    ) -> crate::pac::common::Reg<Mbc0MemnGlbac7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem0BlkCfgW, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w0(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem0BlkNseW0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w1(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem0BlkNseW1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem1BlkCfgW0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_nse_w0(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem1BlkNseW0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_cfg_w0(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem2BlkCfgW0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word."]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_nse_w0(
        self,
    ) -> crate::pac::common::Reg<Mbc0Dom0Mem2BlkNseW0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
}
#[doc = "MBC Memory Block Configuration Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW(pub u32);
impl Mbc0Dom0Mem0BlkCfgW {
    #[doc = "Memory Block Access Control Select for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel(&self, n: usize) -> Mbc0Dom0Mem0BlkCfgWMbacsel {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x07;
        Mbc0Dom0Mem0BlkCfgWMbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B."]
    #[inline(always)]
    pub const fn set_mbacsel(&mut self, n: usize, val: Mbc0Dom0Mem0BlkCfgWMbacsel) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
    #[doc = "NonSecure Enable for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> Mbc0Dom0Mem0BlkCfgWNse {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem0BlkCfgWNse::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B."]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: Mbc0Dom0Mem0BlkCfgWNse) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW {
        Mbc0Dom0Mem0BlkCfgW(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW")
            .field("mbacsel[0]", &self.mbacsel(0usize))
            .field("mbacsel[1]", &self.mbacsel(1usize))
            .field("mbacsel[2]", &self.mbacsel(2usize))
            .field("mbacsel[3]", &self.mbacsel(3usize))
            .field("mbacsel[4]", &self.mbacsel(4usize))
            .field("mbacsel[5]", &self.mbacsel(5usize))
            .field("mbacsel[6]", &self.mbacsel(6usize))
            .field("mbacsel[7]", &self.mbacsel(7usize))
            .field("nse[0]", &self.nse(0usize))
            .field("nse[1]", &self.nse(1usize))
            .field("nse[2]", &self.nse(2usize))
            .field("nse[3]", &self.nse(3usize))
            .field("nse[4]", &self.nse(4usize))
            .field("nse[5]", &self.nse(5usize))
            .field("nse[6]", &self.nse(6usize))
            .field("nse[7]", &self.nse(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW {{ mbacsel[0]: {:?}, mbacsel[1]: {:?}, mbacsel[2]: {:?}, mbacsel[3]: {:?}, mbacsel[4]: {:?}, mbacsel[5]: {:?}, mbacsel[6]: {:?}, mbacsel[7]: {:?}, nse[0]: {:?}, nse[1]: {:?}, nse[2]: {:?}, nse[3]: {:?}, nse[4]: {:?}, nse[5]: {:?}, nse[6]: {:?}, nse[7]: {:?} }}",
            self.mbacsel(0usize),
            self.mbacsel(1usize),
            self.mbacsel(2usize),
            self.mbacsel(3usize),
            self.mbacsel(4usize),
            self.mbacsel(5usize),
            self.mbacsel(6usize),
            self.mbacsel(7usize),
            self.nse(0usize),
            self.nse(1usize),
            self.nse(2usize),
            self.nse(3usize),
            self.nse(4usize),
            self.nse(5usize),
            self.nse(6usize),
            self.nse(7usize)
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkNseW0(pub u32);
impl Mbc0Dom0Mem0BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bit(&self, n: usize) -> Mbc0Dom0Mem0BlkNseW0Bit {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem0BlkNseW0Bit::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[inline(always)]
    pub const fn set_bit(&mut self, n: usize, val: Mbc0Dom0Mem0BlkNseW0Bit) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem0BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkNseW0 {
        Mbc0Dom0Mem0BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkNseW0")
            .field("bit[0]", &self.bit(0usize))
            .field("bit[1]", &self.bit(1usize))
            .field("bit[2]", &self.bit(2usize))
            .field("bit[3]", &self.bit(3usize))
            .field("bit[4]", &self.bit(4usize))
            .field("bit[5]", &self.bit(5usize))
            .field("bit[6]", &self.bit(6usize))
            .field("bit[7]", &self.bit(7usize))
            .field("bit[8]", &self.bit(8usize))
            .field("bit[9]", &self.bit(9usize))
            .field("bit[10]", &self.bit(10usize))
            .field("bit[11]", &self.bit(11usize))
            .field("bit[12]", &self.bit(12usize))
            .field("bit[13]", &self.bit(13usize))
            .field("bit[14]", &self.bit(14usize))
            .field("bit[15]", &self.bit(15usize))
            .field("bit[16]", &self.bit(16usize))
            .field("bit[17]", &self.bit(17usize))
            .field("bit[18]", &self.bit(18usize))
            .field("bit[19]", &self.bit(19usize))
            .field("bit[20]", &self.bit(20usize))
            .field("bit[21]", &self.bit(21usize))
            .field("bit[22]", &self.bit(22usize))
            .field("bit[23]", &self.bit(23usize))
            .field("bit[24]", &self.bit(24usize))
            .field("bit[25]", &self.bit(25usize))
            .field("bit[26]", &self.bit(26usize))
            .field("bit[27]", &self.bit(27usize))
            .field("bit[28]", &self.bit(28usize))
            .field("bit[29]", &self.bit(29usize))
            .field("bit[30]", &self.bit(30usize))
            .field("bit[31]", &self.bit(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkNseW0 {{ bit[0]: {:?}, bit[1]: {:?}, bit[2]: {:?}, bit[3]: {:?}, bit[4]: {:?}, bit[5]: {:?}, bit[6]: {:?}, bit[7]: {:?}, bit[8]: {:?}, bit[9]: {:?}, bit[10]: {:?}, bit[11]: {:?}, bit[12]: {:?}, bit[13]: {:?}, bit[14]: {:?}, bit[15]: {:?}, bit[16]: {:?}, bit[17]: {:?}, bit[18]: {:?}, bit[19]: {:?}, bit[20]: {:?}, bit[21]: {:?}, bit[22]: {:?}, bit[23]: {:?}, bit[24]: {:?}, bit[25]: {:?}, bit[26]: {:?}, bit[27]: {:?}, bit[28]: {:?}, bit[29]: {:?}, bit[30]: {:?}, bit[31]: {:?} }}",
            self.bit(0usize),
            self.bit(1usize),
            self.bit(2usize),
            self.bit(3usize),
            self.bit(4usize),
            self.bit(5usize),
            self.bit(6usize),
            self.bit(7usize),
            self.bit(8usize),
            self.bit(9usize),
            self.bit(10usize),
            self.bit(11usize),
            self.bit(12usize),
            self.bit(13usize),
            self.bit(14usize),
            self.bit(15usize),
            self.bit(16usize),
            self.bit(17usize),
            self.bit(18usize),
            self.bit(19usize),
            self.bit(20usize),
            self.bit(21usize),
            self.bit(22usize),
            self.bit(23usize),
            self.bit(24usize),
            self.bit(25usize),
            self.bit(26usize),
            self.bit(27usize),
            self.bit(28usize),
            self.bit(29usize),
            self.bit(30usize),
            self.bit(31usize)
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkNseW1(pub u32);
impl Mbc0Dom0Mem0BlkNseW1 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bit(&self, n: usize) -> Mbc0Dom0Mem0BlkNseW1Bit {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem0BlkNseW1Bit::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[inline(always)]
    pub const fn set_bit(&mut self, n: usize, val: Mbc0Dom0Mem0BlkNseW1Bit) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem0BlkNseW1 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkNseW1 {
        Mbc0Dom0Mem0BlkNseW1(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkNseW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkNseW1")
            .field("bit[0]", &self.bit(0usize))
            .field("bit[1]", &self.bit(1usize))
            .field("bit[2]", &self.bit(2usize))
            .field("bit[3]", &self.bit(3usize))
            .field("bit[4]", &self.bit(4usize))
            .field("bit[5]", &self.bit(5usize))
            .field("bit[6]", &self.bit(6usize))
            .field("bit[7]", &self.bit(7usize))
            .field("bit[8]", &self.bit(8usize))
            .field("bit[9]", &self.bit(9usize))
            .field("bit[10]", &self.bit(10usize))
            .field("bit[11]", &self.bit(11usize))
            .field("bit[12]", &self.bit(12usize))
            .field("bit[13]", &self.bit(13usize))
            .field("bit[14]", &self.bit(14usize))
            .field("bit[15]", &self.bit(15usize))
            .field("bit[16]", &self.bit(16usize))
            .field("bit[17]", &self.bit(17usize))
            .field("bit[18]", &self.bit(18usize))
            .field("bit[19]", &self.bit(19usize))
            .field("bit[20]", &self.bit(20usize))
            .field("bit[21]", &self.bit(21usize))
            .field("bit[22]", &self.bit(22usize))
            .field("bit[23]", &self.bit(23usize))
            .field("bit[24]", &self.bit(24usize))
            .field("bit[25]", &self.bit(25usize))
            .field("bit[26]", &self.bit(26usize))
            .field("bit[27]", &self.bit(27usize))
            .field("bit[28]", &self.bit(28usize))
            .field("bit[29]", &self.bit(29usize))
            .field("bit[30]", &self.bit(30usize))
            .field("bit[31]", &self.bit(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkNseW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkNseW1 {{ bit[0]: {:?}, bit[1]: {:?}, bit[2]: {:?}, bit[3]: {:?}, bit[4]: {:?}, bit[5]: {:?}, bit[6]: {:?}, bit[7]: {:?}, bit[8]: {:?}, bit[9]: {:?}, bit[10]: {:?}, bit[11]: {:?}, bit[12]: {:?}, bit[13]: {:?}, bit[14]: {:?}, bit[15]: {:?}, bit[16]: {:?}, bit[17]: {:?}, bit[18]: {:?}, bit[19]: {:?}, bit[20]: {:?}, bit[21]: {:?}, bit[22]: {:?}, bit[23]: {:?}, bit[24]: {:?}, bit[25]: {:?}, bit[26]: {:?}, bit[27]: {:?}, bit[28]: {:?}, bit[29]: {:?}, bit[30]: {:?}, bit[31]: {:?} }}",
            self.bit(0usize),
            self.bit(1usize),
            self.bit(2usize),
            self.bit(3usize),
            self.bit(4usize),
            self.bit(5usize),
            self.bit(6usize),
            self.bit(7usize),
            self.bit(8usize),
            self.bit(9usize),
            self.bit(10usize),
            self.bit(11usize),
            self.bit(12usize),
            self.bit(13usize),
            self.bit(14usize),
            self.bit(15usize),
            self.bit(16usize),
            self.bit(17usize),
            self.bit(18usize),
            self.bit(19usize),
            self.bit(20usize),
            self.bit(21usize),
            self.bit(22usize),
            self.bit(23usize),
            self.bit(24usize),
            self.bit(25usize),
            self.bit(26usize),
            self.bit(27usize),
            self.bit(28usize),
            self.bit(29usize),
            self.bit(30usize),
            self.bit(31usize)
        )
    }
}
#[doc = "MBC Memory Block Configuration Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkCfgW0(pub u32);
impl Mbc0Dom0Mem1BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel(&self, n: usize) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x07;
        Mbc0Dom0Mem1BlkCfgW0Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B."]
    #[inline(always)]
    pub const fn set_mbacsel(&mut self, n: usize, val: Mbc0Dom0Mem1BlkCfgW0Mbacsel) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
    #[doc = "NonSecure Enable for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> Mbc0Dom0Mem1BlkCfgW0Nse {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem1BlkCfgW0Nse::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B."]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: Mbc0Dom0Mem1BlkCfgW0Nse) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem1BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkCfgW0 {
        Mbc0Dom0Mem1BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkCfgW0")
            .field("mbacsel[0]", &self.mbacsel(0usize))
            .field("mbacsel[1]", &self.mbacsel(1usize))
            .field("mbacsel[2]", &self.mbacsel(2usize))
            .field("mbacsel[3]", &self.mbacsel(3usize))
            .field("mbacsel[4]", &self.mbacsel(4usize))
            .field("mbacsel[5]", &self.mbacsel(5usize))
            .field("mbacsel[6]", &self.mbacsel(6usize))
            .field("mbacsel[7]", &self.mbacsel(7usize))
            .field("nse[0]", &self.nse(0usize))
            .field("nse[1]", &self.nse(1usize))
            .field("nse[2]", &self.nse(2usize))
            .field("nse[3]", &self.nse(3usize))
            .field("nse[4]", &self.nse(4usize))
            .field("nse[5]", &self.nse(5usize))
            .field("nse[6]", &self.nse(6usize))
            .field("nse[7]", &self.nse(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkCfgW0 {{ mbacsel[0]: {:?}, mbacsel[1]: {:?}, mbacsel[2]: {:?}, mbacsel[3]: {:?}, mbacsel[4]: {:?}, mbacsel[5]: {:?}, mbacsel[6]: {:?}, mbacsel[7]: {:?}, nse[0]: {:?}, nse[1]: {:?}, nse[2]: {:?}, nse[3]: {:?}, nse[4]: {:?}, nse[5]: {:?}, nse[6]: {:?}, nse[7]: {:?} }}",
            self.mbacsel(0usize),
            self.mbacsel(1usize),
            self.mbacsel(2usize),
            self.mbacsel(3usize),
            self.mbacsel(4usize),
            self.mbacsel(5usize),
            self.mbacsel(6usize),
            self.mbacsel(7usize),
            self.nse(0usize),
            self.nse(1usize),
            self.nse(2usize),
            self.nse(3usize),
            self.nse(4usize),
            self.nse(5usize),
            self.nse(6usize),
            self.nse(7usize)
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkNseW0(pub u32);
impl Mbc0Dom0Mem1BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bit(&self, n: usize) -> Mbc0Dom0Mem1BlkNseW0Bit {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem1BlkNseW0Bit::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[inline(always)]
    pub const fn set_bit(&mut self, n: usize, val: Mbc0Dom0Mem1BlkNseW0Bit) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem1BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkNseW0 {
        Mbc0Dom0Mem1BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkNseW0")
            .field("bit[0]", &self.bit(0usize))
            .field("bit[1]", &self.bit(1usize))
            .field("bit[2]", &self.bit(2usize))
            .field("bit[3]", &self.bit(3usize))
            .field("bit[4]", &self.bit(4usize))
            .field("bit[5]", &self.bit(5usize))
            .field("bit[6]", &self.bit(6usize))
            .field("bit[7]", &self.bit(7usize))
            .field("bit[8]", &self.bit(8usize))
            .field("bit[9]", &self.bit(9usize))
            .field("bit[10]", &self.bit(10usize))
            .field("bit[11]", &self.bit(11usize))
            .field("bit[12]", &self.bit(12usize))
            .field("bit[13]", &self.bit(13usize))
            .field("bit[14]", &self.bit(14usize))
            .field("bit[15]", &self.bit(15usize))
            .field("bit[16]", &self.bit(16usize))
            .field("bit[17]", &self.bit(17usize))
            .field("bit[18]", &self.bit(18usize))
            .field("bit[19]", &self.bit(19usize))
            .field("bit[20]", &self.bit(20usize))
            .field("bit[21]", &self.bit(21usize))
            .field("bit[22]", &self.bit(22usize))
            .field("bit[23]", &self.bit(23usize))
            .field("bit[24]", &self.bit(24usize))
            .field("bit[25]", &self.bit(25usize))
            .field("bit[26]", &self.bit(26usize))
            .field("bit[27]", &self.bit(27usize))
            .field("bit[28]", &self.bit(28usize))
            .field("bit[29]", &self.bit(29usize))
            .field("bit[30]", &self.bit(30usize))
            .field("bit[31]", &self.bit(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkNseW0 {{ bit[0]: {:?}, bit[1]: {:?}, bit[2]: {:?}, bit[3]: {:?}, bit[4]: {:?}, bit[5]: {:?}, bit[6]: {:?}, bit[7]: {:?}, bit[8]: {:?}, bit[9]: {:?}, bit[10]: {:?}, bit[11]: {:?}, bit[12]: {:?}, bit[13]: {:?}, bit[14]: {:?}, bit[15]: {:?}, bit[16]: {:?}, bit[17]: {:?}, bit[18]: {:?}, bit[19]: {:?}, bit[20]: {:?}, bit[21]: {:?}, bit[22]: {:?}, bit[23]: {:?}, bit[24]: {:?}, bit[25]: {:?}, bit[26]: {:?}, bit[27]: {:?}, bit[28]: {:?}, bit[29]: {:?}, bit[30]: {:?}, bit[31]: {:?} }}",
            self.bit(0usize),
            self.bit(1usize),
            self.bit(2usize),
            self.bit(3usize),
            self.bit(4usize),
            self.bit(5usize),
            self.bit(6usize),
            self.bit(7usize),
            self.bit(8usize),
            self.bit(9usize),
            self.bit(10usize),
            self.bit(11usize),
            self.bit(12usize),
            self.bit(13usize),
            self.bit(14usize),
            self.bit(15usize),
            self.bit(16usize),
            self.bit(17usize),
            self.bit(18usize),
            self.bit(19usize),
            self.bit(20usize),
            self.bit(21usize),
            self.bit(22usize),
            self.bit(23usize),
            self.bit(24usize),
            self.bit(25usize),
            self.bit(26usize),
            self.bit(27usize),
            self.bit(28usize),
            self.bit(29usize),
            self.bit(30usize),
            self.bit(31usize)
        )
    }
}
#[doc = "MBC Memory Block Configuration Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem2BlkCfgW0(pub u32);
impl Mbc0Dom0Mem2BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel(&self, n: usize) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x07;
        Mbc0Dom0Mem2BlkCfgW0Mbacsel::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B."]
    #[inline(always)]
    pub const fn set_mbacsel(&mut self, n: usize, val: Mbc0Dom0Mem2BlkCfgW0Mbacsel) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
    }
    #[doc = "NonSecure Enable for block B."]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> Mbc0Dom0Mem2BlkCfgW0Nse {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem2BlkCfgW0Nse::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B."]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: Mbc0Dom0Mem2BlkCfgW0Nse) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem2BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem2BlkCfgW0 {
        Mbc0Dom0Mem2BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem2BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem2BlkCfgW0")
            .field("mbacsel[0]", &self.mbacsel(0usize))
            .field("mbacsel[1]", &self.mbacsel(1usize))
            .field("mbacsel[2]", &self.mbacsel(2usize))
            .field("mbacsel[3]", &self.mbacsel(3usize))
            .field("mbacsel[4]", &self.mbacsel(4usize))
            .field("mbacsel[5]", &self.mbacsel(5usize))
            .field("mbacsel[6]", &self.mbacsel(6usize))
            .field("mbacsel[7]", &self.mbacsel(7usize))
            .field("nse[0]", &self.nse(0usize))
            .field("nse[1]", &self.nse(1usize))
            .field("nse[2]", &self.nse(2usize))
            .field("nse[3]", &self.nse(3usize))
            .field("nse[4]", &self.nse(4usize))
            .field("nse[5]", &self.nse(5usize))
            .field("nse[6]", &self.nse(6usize))
            .field("nse[7]", &self.nse(7usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem2BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem2BlkCfgW0 {{ mbacsel[0]: {:?}, mbacsel[1]: {:?}, mbacsel[2]: {:?}, mbacsel[3]: {:?}, mbacsel[4]: {:?}, mbacsel[5]: {:?}, mbacsel[6]: {:?}, mbacsel[7]: {:?}, nse[0]: {:?}, nse[1]: {:?}, nse[2]: {:?}, nse[3]: {:?}, nse[4]: {:?}, nse[5]: {:?}, nse[6]: {:?}, nse[7]: {:?} }}",
            self.mbacsel(0usize),
            self.mbacsel(1usize),
            self.mbacsel(2usize),
            self.mbacsel(3usize),
            self.mbacsel(4usize),
            self.mbacsel(5usize),
            self.mbacsel(6usize),
            self.mbacsel(7usize),
            self.nse(0usize),
            self.nse(1usize),
            self.nse(2usize),
            self.nse(3usize),
            self.nse(4usize),
            self.nse(5usize),
            self.nse(6usize),
            self.nse(7usize)
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem2BlkNseW0(pub u32);
impl Mbc0Dom0Mem2BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn bit(&self, n: usize) -> Mbc0Dom0Mem2BlkNseW0Bit {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Mbc0Dom0Mem2BlkNseW0Bit::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]."]
    #[inline(always)]
    pub const fn set_bit(&mut self, n: usize, val: Mbc0Dom0Mem2BlkNseW0Bit) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Mbc0Dom0Mem2BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem2BlkNseW0 {
        Mbc0Dom0Mem2BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem2BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem2BlkNseW0")
            .field("bit[0]", &self.bit(0usize))
            .field("bit[1]", &self.bit(1usize))
            .field("bit[2]", &self.bit(2usize))
            .field("bit[3]", &self.bit(3usize))
            .field("bit[4]", &self.bit(4usize))
            .field("bit[5]", &self.bit(5usize))
            .field("bit[6]", &self.bit(6usize))
            .field("bit[7]", &self.bit(7usize))
            .field("bit[8]", &self.bit(8usize))
            .field("bit[9]", &self.bit(9usize))
            .field("bit[10]", &self.bit(10usize))
            .field("bit[11]", &self.bit(11usize))
            .field("bit[12]", &self.bit(12usize))
            .field("bit[13]", &self.bit(13usize))
            .field("bit[14]", &self.bit(14usize))
            .field("bit[15]", &self.bit(15usize))
            .field("bit[16]", &self.bit(16usize))
            .field("bit[17]", &self.bit(17usize))
            .field("bit[18]", &self.bit(18usize))
            .field("bit[19]", &self.bit(19usize))
            .field("bit[20]", &self.bit(20usize))
            .field("bit[21]", &self.bit(21usize))
            .field("bit[22]", &self.bit(22usize))
            .field("bit[23]", &self.bit(23usize))
            .field("bit[24]", &self.bit(24usize))
            .field("bit[25]", &self.bit(25usize))
            .field("bit[26]", &self.bit(26usize))
            .field("bit[27]", &self.bit(27usize))
            .field("bit[28]", &self.bit(28usize))
            .field("bit[29]", &self.bit(29usize))
            .field("bit[30]", &self.bit(30usize))
            .field("bit[31]", &self.bit(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem2BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem2BlkNseW0 {{ bit[0]: {:?}, bit[1]: {:?}, bit[2]: {:?}, bit[3]: {:?}, bit[4]: {:?}, bit[5]: {:?}, bit[6]: {:?}, bit[7]: {:?}, bit[8]: {:?}, bit[9]: {:?}, bit[10]: {:?}, bit[11]: {:?}, bit[12]: {:?}, bit[13]: {:?}, bit[14]: {:?}, bit[15]: {:?}, bit[16]: {:?}, bit[17]: {:?}, bit[18]: {:?}, bit[19]: {:?}, bit[20]: {:?}, bit[21]: {:?}, bit[22]: {:?}, bit[23]: {:?}, bit[24]: {:?}, bit[25]: {:?}, bit[26]: {:?}, bit[27]: {:?}, bit[28]: {:?}, bit[29]: {:?}, bit[30]: {:?}, bit[31]: {:?} }}",
            self.bit(0usize),
            self.bit(1usize),
            self.bit(2usize),
            self.bit(3usize),
            self.bit(4usize),
            self.bit(5usize),
            self.bit(6usize),
            self.bit(7usize),
            self.bit(8usize),
            self.bit(9usize),
            self.bit(10usize),
            self.bit(11usize),
            self.bit(12usize),
            self.bit(13usize),
            self.bit(14usize),
            self.bit(15usize),
            self.bit(16usize),
            self.bit(17usize),
            self.bit(18usize),
            self.bit(19usize),
            self.bit(20usize),
            self.bit(21usize),
            self.bit(22usize),
            self.bit(23usize),
            self.bit(24usize),
            self.bit(25usize),
            self.bit(26usize),
            self.bit(27usize),
            self.bit(28usize),
            self.bit(29usize),
            self.bit(30usize),
            self.bit(31usize)
        )
    }
}
#[doc = "MBC Global Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem0Glbcfg(pub u32);
impl Mbc0Mem0Glbcfg {
    #[doc = "Number of blocks in this memory."]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory."]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block."]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block."]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem0Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem0Glbcfg {
        Mbc0Mem0Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem0Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem0Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem0Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem0Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem1Glbcfg(pub u32);
impl Mbc0Mem1Glbcfg {
    #[doc = "Number of blocks in this memory."]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory."]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block."]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block."]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem1Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem1Glbcfg {
        Mbc0Mem1Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem1Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem1Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem1Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem1Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem2Glbcfg(pub u32);
impl Mbc0Mem2Glbcfg {
    #[doc = "Number of blocks in this memory."]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory."]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block."]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block."]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem2Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem2Glbcfg {
        Mbc0Mem2Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem2Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem2Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem2Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem2Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem3Glbcfg(pub u32);
impl Mbc0Mem3Glbcfg {
    #[doc = "Number of blocks in this memory."]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory."]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block."]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block."]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Clear Error."]
    #[must_use]
    #[inline(always)]
    pub const fn clre(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Clear Error."]
    #[inline(always)]
    pub const fn set_clre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Mbc0Mem3Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem3Glbcfg {
        Mbc0Mem3Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem3Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem3Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .field("clre", &self.clre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem3Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem3Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?}, clre: {=u8:?} }}",
            self.nblks(),
            self.size_log2(),
            self.clre()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac0(pub u32);
impl Mbc0MemnGlbac0 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mbc0MemnGlbac0 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac0 {
        Mbc0MemnGlbac0(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac0")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac0 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac1(pub u32);
impl Mbc0MemnGlbac1 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac1 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac1 {
        Mbc0MemnGlbac1(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac1")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac1 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac2(pub u32);
impl Mbc0MemnGlbac2 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac2 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac2 {
        Mbc0MemnGlbac2(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac2")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac2 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac3(pub u32);
impl Mbc0MemnGlbac3 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac3 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac3 {
        Mbc0MemnGlbac3(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac3")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac3 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac4(pub u32);
impl Mbc0MemnGlbac4 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac4 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac4 {
        Mbc0MemnGlbac4(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac4")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac4 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac5(pub u32);
impl Mbc0MemnGlbac5 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac5 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac5 {
        Mbc0MemnGlbac5(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac5")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac5 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac6(pub u32);
impl Mbc0MemnGlbac6 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac6 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac6 {
        Mbc0MemnGlbac6(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac6")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac6 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac7(pub u32);
impl Mbc0MemnGlbac7 {
    #[doc = "NonsecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute."]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write."]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read."]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute."]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write."]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read."]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute."]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write."]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write."]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read."]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read."]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute."]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute."]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write."]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write."]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read."]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read."]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac7 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac7 {
        Mbc0MemnGlbac7(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac7")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac7 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkClr(pub u32);
impl Mbc0NseBlkClr {
    #[doc = "Write-1 Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn w1clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write-1 Clear."]
    #[inline(always)]
    pub const fn set_w1clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mbc0NseBlkClr {
    #[inline(always)]
    fn default() -> Mbc0NseBlkClr {
        Mbc0NseBlkClr(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkClr")
            .field("w1clr", &self.w1clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mbc0NseBlkClr {{ w1clr: {=u32:?} }}", self.w1clr())
    }
}
#[doc = "MBC NonSecure Enable Block Clear All."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkClrAll(pub u32);
impl Mbc0NseBlkClrAll {
    #[doc = "Memory Select."]
    #[must_use]
    #[inline(always)]
    pub const fn memsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select."]
    #[inline(always)]
    pub const fn set_memsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select."]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> Mbc0NseBlkClrAllDidSel0 {
        let val = (self.0 >> 16usize) & 0x01;
        Mbc0NseBlkClrAllDidSel0::from_bits(val as u8)
    }
    #[doc = "DID Select."]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: Mbc0NseBlkClrAllDidSel0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Mbc0NseBlkClrAll {
    #[inline(always)]
    fn default() -> Mbc0NseBlkClrAll {
        Mbc0NseBlkClrAll(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkClrAll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkClrAll")
            .field("memsel", &self.memsel())
            .field("did_sel0", &self.did_sel0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkClrAll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0NseBlkClrAll {{ memsel: {=u8:?}, did_sel0: {:?} }}",
            self.memsel(),
            self.did_sel0()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkIndex(pub u32);
impl Mbc0NseBlkIndex {
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[must_use]
    #[inline(always)]
    pub const fn wndx(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[inline(always)]
    pub const fn set_wndx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Memory Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mem_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select."]
    #[inline(always)]
    pub const fn set_mem_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select."]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> Mbc0NseBlkIndexDidSel0 {
        let val = (self.0 >> 16usize) & 0x01;
        Mbc0NseBlkIndexDidSel0::from_bits(val as u8)
    }
    #[doc = "DID Select."]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: Mbc0NseBlkIndexDidSel0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Auto Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn ai(&self) -> Ai {
        let val = (self.0 >> 31usize) & 0x01;
        Ai::from_bits(val as u8)
    }
    #[doc = "Auto Increment."]
    #[inline(always)]
    pub const fn set_ai(&mut self, val: Ai) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0NseBlkIndex {
    #[inline(always)]
    fn default() -> Mbc0NseBlkIndex {
        Mbc0NseBlkIndex(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkIndex")
            .field("wndx", &self.wndx())
            .field("mem_sel", &self.mem_sel())
            .field("did_sel0", &self.did_sel0())
            .field("ai", &self.ai())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkIndex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0NseBlkIndex {{ wndx: {=u8:?}, mem_sel: {=u8:?}, did_sel0: {:?}, ai: {:?} }}",
            self.wndx(),
            self.mem_sel(),
            self.did_sel0(),
            self.ai()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkSet(pub u32);
impl Mbc0NseBlkSet {
    #[doc = "Write-1 Set."]
    #[must_use]
    #[inline(always)]
    pub const fn w1set(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write-1 Set."]
    #[inline(always)]
    pub const fn set_w1set(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mbc0NseBlkSet {
    #[inline(always)]
    fn default() -> Mbc0NseBlkSet {
        Mbc0NseBlkSet(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkSet")
            .field("w1set", &self.w1set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mbc0NseBlkSet {{ w1set: {=u32:?} }}", self.w1set())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ai {
    #[doc = "No effect."]
    Logic0 = 0x0,
    #[doc = "Add 1 to the WNDX field after the register write."]
    Logic1 = 0x01,
}
impl Ai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ai {
    #[inline(always)]
    fn from(val: u8) -> Ai {
        Ai::from_bits(val)
    }
}
impl From<Ai> for u8 {
    #[inline(always)]
    fn from(val: Ai) -> u8 {
        Ai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgWMbacsel {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem0BlkCfgWMbacsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgWMbacsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgWMbacsel {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgWMbacsel {
        Mbc0Dom0Mem0BlkCfgWMbacsel::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgWMbacsel> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgWMbacsel) -> u8 {
        Mbc0Dom0Mem0BlkCfgWMbacsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkCfgWNse {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkCfgWNse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkCfgWNse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkCfgWNse {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkCfgWNse {
        Mbc0Dom0Mem0BlkCfgWNse::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkCfgWNse> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkCfgWNse) -> u8 {
        Mbc0Dom0Mem0BlkCfgWNse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW0Bit {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW0Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW0Bit {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW0Bit {
        Mbc0Dom0Mem0BlkNseW0Bit::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW0Bit> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW0Bit) -> u8 {
        Mbc0Dom0Mem0BlkNseW0Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem0BlkNseW1Bit {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem0BlkNseW1Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem0BlkNseW1Bit {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem0BlkNseW1Bit {
        Mbc0Dom0Mem0BlkNseW1Bit::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem0BlkNseW1Bit> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem0BlkNseW1Bit) -> u8 {
        Mbc0Dom0Mem0BlkNseW1Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Mbacsel {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem1BlkCfgW0Mbacsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Mbacsel {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Mbacsel {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Mbacsel> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Mbacsel) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Mbacsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkCfgW0Nse {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkCfgW0Nse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkCfgW0Nse {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkCfgW0Nse {
        Mbc0Dom0Mem1BlkCfgW0Nse::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkCfgW0Nse> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkCfgW0Nse) -> u8 {
        Mbc0Dom0Mem1BlkCfgW0Nse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem1BlkNseW0Bit {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem1BlkNseW0Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem1BlkNseW0Bit {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem1BlkNseW0Bit {
        Mbc0Dom0Mem1BlkNseW0Bit::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem1BlkNseW0Bit> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem1BlkNseW0Bit) -> u8 {
        Mbc0Dom0Mem1BlkNseW0Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Mbacsel {
    #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B."]
    Glbac0 = 0x0,
    #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B."]
    Glbac1 = 0x01,
    #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B."]
    Glbac2 = 0x02,
    #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B."]
    Glbac3 = 0x03,
    #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B."]
    Glbac4 = 0x04,
    #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B."]
    Glbac5 = 0x05,
    #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B."]
    Glbac6 = 0x06,
    #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B."]
    Glbac7 = 0x07,
}
impl Mbc0Dom0Mem2BlkCfgW0Mbacsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Mbacsel {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Mbacsel {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Mbacsel> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Mbacsel) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Mbacsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkCfgW0Nse {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkCfgW0Nse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkCfgW0Nse {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkCfgW0Nse {
        Mbc0Dom0Mem2BlkCfgW0Nse::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkCfgW0Nse> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkCfgW0Nse) -> u8 {
        Mbc0Dom0Mem2BlkCfgW0Nse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0Dom0Mem2BlkNseW0Bit {
    #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\]), nonsecure accesses to block B are not allowed."]
    Allowed = 0x0,
    #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww\\[MBACSEL\\])."]
    Notallowed = 0x01,
}
impl Mbc0Dom0Mem2BlkNseW0Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0Dom0Mem2BlkNseW0Bit {
    #[inline(always)]
    fn from(val: u8) -> Mbc0Dom0Mem2BlkNseW0Bit {
        Mbc0Dom0Mem2BlkNseW0Bit::from_bits(val)
    }
}
impl From<Mbc0Dom0Mem2BlkNseW0Bit> for u8 {
    #[inline(always)]
    fn from(val: Mbc0Dom0Mem2BlkNseW0Bit) -> u8 {
        Mbc0Dom0Mem2BlkNseW0Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0NseBlkClrAllDidSel0 {
    #[doc = "No effect."]
    Logic0 = 0x0,
    #[doc = "Clear all NSE bits for this domain."]
    Logic1 = 0x01,
}
impl Mbc0NseBlkClrAllDidSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0NseBlkClrAllDidSel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0NseBlkClrAllDidSel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0NseBlkClrAllDidSel0 {
        Mbc0NseBlkClrAllDidSel0::from_bits(val)
    }
}
impl From<Mbc0NseBlkClrAllDidSel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0NseBlkClrAllDidSel0) -> u8 {
        Mbc0NseBlkClrAllDidSel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc0NseBlkIndexDidSel0 {
    #[doc = "No effect."]
    Logic0 = 0x0,
    #[doc = "Selects NSE bits for this domain."]
    Logic1 = 0x01,
}
impl Mbc0NseBlkIndexDidSel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc0NseBlkIndexDidSel0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc0NseBlkIndexDidSel0 {
    #[inline(always)]
    fn from(val: u8) -> Mbc0NseBlkIndexDidSel0 {
        Mbc0NseBlkIndexDidSel0::from_bits(val)
    }
}
impl From<Mbc0NseBlkIndexDidSel0> for u8 {
    #[inline(always)]
    fn from(val: Mbc0NseBlkIndexDidSel0) -> u8 {
        Mbc0NseBlkIndexDidSel0::to_bits(val)
    }
}
