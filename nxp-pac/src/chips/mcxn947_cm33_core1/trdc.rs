#[doc = "TRDC"]
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
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem0_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem0Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem1_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem1Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem2_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem2Glbcfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MBC Global Configuration Register"]
    #[inline(always)]
    pub const fn mbc0_mem3_glbcfg(
        self,
    ) -> crate::common::Reg<regs::Mbc0Mem3Glbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Index"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_index(
        self,
    ) -> crate::common::Reg<regs::Mbc0NseBlkIndex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Set"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_set(
        self,
    ) -> crate::common::Reg<regs::Mbc0NseBlkSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr(
        self,
    ) -> crate::common::Reg<regs::Mbc0NseBlkClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "MBC NonSecure Enable Block Clear All"]
    #[inline(always)]
    pub const fn mbc0_nse_blk_clr_all(
        self,
    ) -> crate::common::Reg<regs::Mbc0NseBlkClrAll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac0(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac1(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac2(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac3(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac4(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac5(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac6(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "MBC Global Access Control"]
    #[inline(always)]
    pub const fn mbc0_memn_glbac7(
        self,
    ) -> crate::common::Reg<regs::Mbc0MemnGlbac7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w1(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w2(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w3(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w4(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w5(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w6(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_cfg_w7(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkCfgW7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkNseW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem0_blk_nse_w1(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem0BlkNseW1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem1BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem1_blk_nse_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem1BlkNseW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_cfg_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem2BlkCfgW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[inline(always)]
    pub const fn mbc0_dom0_mem2_blk_nse_w0(
        self,
    ) -> crate::common::Reg<regs::Mbc0Dom0Mem2BlkNseW0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
