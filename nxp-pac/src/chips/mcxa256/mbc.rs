#[doc = "TRDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc {
    ptr: *mut u8,
}
unsafe impl Send for Mbc {}
unsafe impl Sync for Mbc {}
impl Mbc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem0_glbcfg(self) -> crate::common::Reg<regs::Mem0Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem1_glbcfg(self) -> crate::common::Reg<regs::Mem1Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem2_glbcfg(self) -> crate::common::Reg<regs::Mem2Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem3_glbcfg(self) -> crate::common::Reg<regs::Mem3Glbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac0(self) -> crate::common::Reg<regs::MemnGlbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac1(self) -> crate::common::Reg<regs::MemnGlbac1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac2(self) -> crate::common::Reg<regs::MemnGlbac2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac3(self) -> crate::common::Reg<regs::MemnGlbac3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac4(self) -> crate::common::Reg<regs::MemnGlbac4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac5(self) -> crate::common::Reg<regs::MemnGlbac5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac6(self) -> crate::common::Reg<regs::MemnGlbac6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac7(self) -> crate::common::Reg<regs::MemnGlbac7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_wn(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MemnBlkCfgW, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::MemnBlkCfgW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<regs::MemnBlkCfgW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::MemnBlkCfgW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
